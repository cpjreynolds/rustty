use std::ops::{Index, IndexMut};
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::fs::File;
use std::os::unix::io::{AsRawFd, RawFd};
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::collections::VecDeque;

use nix::sys::termios;
use nix::sys::termios::{IGNBRK, BRKINT, PARMRK, ISTRIP, INLCR, IGNCR, ICRNL, IXON};
use nix::sys::termios::{OPOST, ECHO, ECHONL, ICANON, ISIG, IEXTEN, CSIZE, PARENB, CS8};
use nix::sys::termios::{VMIN, VTIME};
use nix::sys::termios::SetArg;
use nix::sys::signal;
use nix::sys::signal::{SockFlag, SigSet};
use nix::sys::signal::signal::SIGWINCH;
use nix::sys::ioctl;
use nix::sys::epoll::{epoll_create, epoll_ctl, epoll_wait};
use nix::sys::epoll::{EpollOp, EpollEvent, EpollEventKind};
use nix::sys::epoll;
use nix::errno::Errno;

use util::errors::Error;
use core::cellbuffer::{CellBuffer, Cell, Style, Color, Attr};
use core::input::Event;
use core::position::{Position, Coordinate, Cursor, Pair};
use core::driver::{
    DevFn,
    Driver,
};

/// Set to true by the sigwinch handler. Reset to false when buffers are resized.
static SIGWINCH_STATUS: AtomicBool = ATOMIC_BOOL_INIT;

/// Ensures that there is only ever one Terminal object at any one time.
/// Set to true on creation of a Terminal object.
/// Reset to false when terminal object goes out of scope.
static RUSTTY_STATUS: AtomicBool = ATOMIC_BOOL_INIT;

/// TIOCGWINSZ constant.
#[cfg(target_os="macos")]
const TIOCGWINSZ: u64 = 0x40087468;
#[cfg(target_os="linux")]
const TIOCGWINSZ: u64 = 0x00005413;

type OutBuffer = Vec<u8>;
type EventBuffer = VecDeque<Event>;

/// A representation of the current terminal window.
///
/// Only one `Terminal` object can exist at any one time, `Terminal::new()` will return an `Error`
/// if a `Terminal` object already exists.
/// When a `Terminal` goes out of scope it resets the underlying terminal to its original state.
///
/// # Examples
///
/// ```no_run
/// use rustty::{Terminal, Cell, Style, Color};
///
/// // Construct a new Terminal.
/// let mut term = Terminal::new().unwrap();
///
/// // Terminals can be indexed to access specific cells.
/// // Indices are by row then column, corresponding to a cell's y and x coordinates.
/// term[(0, 0)] = Cell::with_char('x');
/// assert_eq!(term[(0, 0)].ch(), 'x');
///
/// term[(0, 1)].set_bg(Style::with_color(Color::Red));
/// assert_eq!(term[(0, 1)].bg(), Style::with_color(Color::Red));
///
/// term[(0, 2)].fg_mut().set_color(Color::Blue);
/// assert_eq!(term[(0, 2)].fg().color(), Color::Blue);
/// ```
pub struct Terminal {
    orig_tios: termios::Termios, // Original underlying terminal state.
    tty: File, // Underlying terminal file.
    rawtty: RawFd, // Raw file descriptor of underlying terminal file.
    epfd: RawFd, // Epoll file descriptor.
    cols: usize, // Number of columns in the terminal window.
    rows: usize, // Number of rows in the terminal window.
    driver: Driver,
    backbuffer: CellBuffer, // Internal backbuffer.
    frontbuffer: CellBuffer, // Internal frontbuffer.
    outbuffer: OutBuffer, // Internal output buffer.
    eventbuffer: EventBuffer, // Event buffer.
    lastfg: Style, // Last foreground style written to the output buffer.
    lastbg: Style, // Last background style written to the input buffer.
    cursor: Cursor, // Current cursor position.
}

impl Terminal {
    /// Constructs a new `Terminal` using the default `Cell` as a blank.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::Terminal;
    ///
    /// let mut term = Terminal::new().unwrap();
    /// assert_eq!(term[0][0].ch(), ' ');
    /// ```
    pub fn new() -> Result<Terminal, Error> {
        Terminal::with_cell(Cell::default())
    }

    /// Constructs a new `Terminal` with each cell set to the given `char` and the default
    /// `Style`s.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::{Terminal, Cell};
    ///
    /// let mut term = Terminal::with_char('x').unwrap();
    /// assert_eq!(term[0][0].ch(), 'x');
    /// ```
    pub fn with_char(ch: char) -> Result<Terminal, Error> {
        Terminal::with_cell(Cell::with_char(ch))
    }

    /// Constructs a new `Terminal` with each cell set to the given `Style`s and a blank `char`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::{Terminal, Cell, Style, Color, Attr};
    ///
    /// let style1 = Style::with_color(Color::Blue);
    /// let style2 = Style::with_attr(Attr::Reverse);
    ///
    /// let mut term = Terminal::with_styles(style1, style2).unwrap();
    /// assert_eq!(term[0][0].fg(), Style::with_color(Color::Blue));
    /// assert_eq!(term[0][0].bg(), Style::with_attr(Attr::Reverse));
    /// assert_eq!(term[0][0].ch(), ' ');
    /// ```
    pub fn with_styles(fg: Style, bg: Style) -> Result<Terminal, Error> {
        Terminal::with_cell(Cell::with_styles(fg, bg))
    }

    /// Creates a new `Terminal` using the given cell as a blank.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::{Terminal, Cell};
    ///
    /// let cell = Cell::with_char('x');
    ///
    /// let mut term = Terminal::with_cell(cell).unwrap();
    /// assert_eq!(term[0][0].ch(), 'x');
    /// ```
    pub fn with_cell(cell: Cell) -> Result<Terminal, Error> {
        // Make sure there is only ever one instance.
        if RUSTTY_STATUS.compare_and_swap(false, true, Ordering::SeqCst) {
            return Err(Error::new("Rustty already initialized"))
        }

        let driver = try!(Driver::new());

        // Open the terminal file for the controlling process.
        let tty = try!(OpenOptions::new()
            .write(true)
            .read(true)
            .open("/dev/tty"));

        // Get the raw file descriptor for the terminal file to use with system calls.
        let rawtty = tty.as_raw_fd();

        // Set up the signal handler for SIGWINCH, which will notify us when the window size has
        // changed; it does this by setting SIGWINCH_STATUS to 'true'.
        let sa_winch = signal::SigAction::new(sigwinch_handler, SockFlag::empty(), SigSet::empty());
        try!(unsafe {
            signal::sigaction(SIGWINCH, &sa_winch)
        });

        // Get the original state of the terminal so we can restore it on drop.
        let orig_tios = try!(termios::tcgetattr(rawtty));

        // Make a mutable clone of the terminal state so we can modify parameters.
        let mut tios = orig_tios.clone();

        // Set required terminal parameters.
        tios.c_iflag = tios.c_iflag & !(IGNBRK | BRKINT | PARMRK | ISTRIP |
                                        INLCR | IGNCR | ICRNL | IXON);
        tios.c_oflag = tios.c_oflag & !OPOST;
        tios.c_lflag = tios.c_lflag & !(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
        tios.c_cflag = tios.c_cflag & !(CSIZE | PARENB);
        tios.c_cflag = tios.c_cflag | CS8;
        tios.c_cc[VMIN] = 0;
        tios.c_cc[VTIME] = 0;

        // Make the system call to change terminal parameters.
        try!(termios::tcsetattr(rawtty, SetArg::TCSAFLUSH, &tios));

        let epfd = try!(epoll_create());
        let epev = EpollEvent {
            events: epoll::EPOLLIN,
            data: 0,
        };
        try!(epoll_ctl(epfd, EpollOp::EpollCtlAdd, rawtty, &epev));

        // Create the terminal object to hold all of our required state.
        let mut terminal = Terminal {
            orig_tios: orig_tios,
            tty: tty,
            rawtty: rawtty,
            epfd: epfd,
            cols: 0,
            rows: 0,
            driver: driver,
            backbuffer: CellBuffer::new(0, 0, cell),
            frontbuffer: CellBuffer::new(0, 0, cell),
            outbuffer: OutBuffer::with_capacity(32 * 1024),
            eventbuffer: EventBuffer::with_capacity(128),
            lastfg: cell.fg(),
            lastbg: cell.bg(),
            cursor: Cursor::new(),
        };

        // Switch to alternate screen buffer. Writes the control code to the output buffer.
        try!(terminal.outbuffer.write_all(&terminal.driver.get(DevFn::EnterCa)));

        // Hide cursor. Writes the control code to the output buffer.
        try!(terminal.outbuffer.write_all(&terminal.driver.get(DevFn::HideCursor)));

        // Resize the buffers to the size of the underlying terminals. Using the given cell as a
        // blank.
        try!(terminal.resize_with_cell(cell));

        // Return the initialized terminal object.
        Ok(terminal)
    }

    /// Swaps buffers to display the current backbuffer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::Terminal;
    ///
    /// let mut term = Terminal::new().unwrap();
    /// term.swap_buffers().unwrap();
    /// ```
    pub fn swap_buffers(&mut self) -> Result<(), Error> {
        // Check whether the window has been resized; if it has then update and resize the buffers.
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize());
        }

        // Invalidate the last cursor position.
        self.cursor.invalidate_last_pos();

        for y in 0..self.rows() {
            if self.frontbuffer[y] == self.backbuffer[y] {
                continue; // Don't redraw draw columns that haven't changed.
            }
            for x in 0..self.cols() {
                if self.frontbuffer[y][x] == self.backbuffer[y][x] {
                    continue; // Don't redraw cells that haven't changed.
                } else {
                    let cell = self.backbuffer[y][x];
                    try!(self.send_style(cell.fg(), cell.bg()));
                    try!(self.send_char(Coordinate::Valid((x, y)), cell.ch()));
                    self.frontbuffer[y][x] = cell;
                }
            }
        }
        try!(self.send_cursor());
        try!(self.flush());
        Ok(())
    }

    /// Returns the width of the terminal in columns.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::Terminal;
    ///
    /// let mut term = Terminal::new().unwrap();
    /// let width = term.cols();
    /// ```
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns the height of the terminal in rows.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::Terminal;
    ///
    /// let mut term = Terminal::new().unwrap();
    /// let height = term.rows();
    /// ```
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the size of the terminal as (cols, rows).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::Terminal;
    ///
    /// let mut term = Terminal::new().unwrap();
    /// let size = term.size();
    /// assert_eq!(size, (term.cols(), term.rows()));
    /// ```
    pub fn size(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }

    /// Clears the internal backbuffer with the default cell.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::{Terminal, Cell};
    ///
    /// let mut term = Terminal::with_char('x').unwrap();
    /// assert_eq!(term[0][0].ch(), 'x');
    ///
    /// term.clear().unwrap();
    /// assert_eq!(term[0][0].ch(), ' ');
    /// ```
    pub fn clear(&mut self) -> Result<(), Error> {
        // Check whether the window has been resized; if it has then update and resize the buffers.
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize());
        }
        self.backbuffer.clear(Cell::default());
        Ok(())
    }

    /// Clears the internal backbuffer with the given `char` and the default `Style`s.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::{Terminal, Cell};
    ///
    /// let mut term = Terminal::with_char('x').unwrap();
    /// assert_eq!(term[0][0].ch(), 'x');
    ///
    /// term.clear_with_char('y').unwrap();
    /// assert_eq!(term[0][0].ch(), 'y');
    /// ```
    pub fn clear_with_char(&mut self, ch: char) -> Result<(), Error> {
        // Check whether the window has been resized; if it has then update and resize the buffers.
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize());
        }
        self.backbuffer.clear(Cell::with_char(ch));
        Ok(())
    }

    /// Clears the internal backbuffer with the given `Style`s and a blank `char`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::{Terminal, Cell, Style, Color};
    ///
    /// let mut style1 = Style::with_color(Color::Blue);
    /// let mut style2 = Style::with_color(Color::Red);
    ///
    /// let mut term = Terminal::with_styles(style1, style2).unwrap();
    /// assert_eq!(term[0][0].fg(), Style::with_color(Color::Blue));
    /// assert_eq!(term[0][0].bg(), Style::with_color(Color::Red));
    ///
    /// term.clear_with_styles(style2, style1).unwrap();
    /// assert_eq!(term[0][0].fg(), Style::with_color(Color::Red));
    /// assert_eq!(term[0][0].bg(), Style::with_color(Color::Blue));
    /// ```
    pub fn clear_with_styles(&mut self, fg: Style, bg: Style) -> Result<(), Error> {
        // Check whether the window has been resized; if it has then update and resize the buffers.
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize());
        }
        self.backbuffer.clear(Cell::with_styles(fg, bg));
        Ok(())
    }

    /// Clears the internal backbuffer using the given `Cell` as a blank.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::{Terminal, Cell};
    ///
    /// let cell1 = Cell::with_char('x');
    /// let cell2 = Cell::with_char('y');
    ///
    /// let mut term = Terminal::with_cell(cell1).unwrap();
    /// assert_eq!(term[0][0].ch(), 'x');
    ///
    /// term.clear_with_cell(cell2).unwrap();
    /// assert_eq!(term[0][0].ch(), 'y');
    /// ```
    pub fn clear_with_cell(&mut self, cell: Cell) -> Result<(), Error> {
        // Check whether the window has been resized; if it has then update and resize the buffers.
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize());
        }
        self.backbuffer.clear(cell);
        Ok(())
    }

    /// Checks whether the underlying window size has changed and the buffers have not been
    /// resized yet. If this method returns `true` the next call to `swap_buffers()` or a `clear()`
    /// method is guaranteed to resize the buffers unless a call to a `try_resize()` method is
    /// made.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::{Terminal, Cell};
    ///
    /// let mut term = Terminal::new().unwrap();
    ///
    /// let will_resize = term.check_resize();
    ///
    /// // If will_resize == true, swap_buffers() will resize the buffers.
    /// term.swap_buffers().unwrap();
    /// // So will clear().
    /// term.clear().unwrap();
    ///
    /// // Unless try_resize() is called.
    /// term.try_resize().unwrap();
    /// ```
    pub fn check_resize(&self) -> bool {
        SIGWINCH_STATUS.load(Ordering::SeqCst)
    }

    /// Resizes the buffers if the underlying terminal window size has changed, using the default
    /// `Cell` as a blank.
    ///
    /// This method will be called automatically on each call to `swap_buffers()` or a `clear()`
    /// method.
    ///
    /// This method is guaranteed to resize the buffers if a call to `check_resize()` returns
    /// `true` and neither `swap_buffers()` nor a `clear()` method has been called since.
    ///
    /// Returns `Some((cols, rows))` if the buffers were resized and `None` if no resize was
    /// performed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::Terminal;
    ///
    /// let mut term = Terminal::new().unwrap();
    ///
    /// // If new_size == Some(T) then T is the new size of the terminal.
    /// // If new_size == None then the terminal has not resized.
    /// let new_size = term.try_resize().unwrap();
    /// ```
    pub fn try_resize(&mut self) -> Result<Option<(usize, usize)>, Error> {
        self.try_resize_with_cell(Cell::default())
    }

    /// Resizes the buffers if the underlying terminal window size has changed, using the given
    /// `char` and the default `Style`s as a blank.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::Terminal;
    ///
    /// let mut term = Terminal::new().unwrap();
    ///
    /// // If new_size == Some(T) then T is the new size of the terminal.
    /// // If new_size == None then the terminal has not resized.
    /// let new_size = term.try_resize_with_char('x').unwrap();
    /// ```
    pub fn try_resize_with_char(&mut self, ch: char) -> Result<Option<(usize, usize)>, Error> {
        self.try_resize_with_cell(Cell::with_char(ch))
    }

    /// Resizes the buffers if the underlying terminal window size has changed, using the given
    /// `Style`s and a blank `char` as a blank.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::{Terminal, Style, Color};
    ///
    /// let style = Style::with_color(Color::Red);
    ///
    /// let mut term = Terminal::new().unwrap();
    ///
    /// // If new_size == Some(T) then T is the new size of the terminal.
    /// // If new_size == None then the terminal has not resized.
    /// let new_size = term.try_resize_with_styles(style, style).unwrap();
    /// ```
    pub fn try_resize_with_styles(&mut self,
                                  fg: Style,
                                  bg: Style) -> Result<Option<(usize, usize)>, Error> {
        self.try_resize_with_cell(Cell::with_styles(fg, bg))
    }

    /// Resizes the buffers if the underlying terminal window size has changed, using the given
    /// `Cell` as a blank.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::{Terminal, Cell};
    ///
    /// let cell = Cell::with_char('x');
    ///
    /// let mut term = Terminal::new().unwrap();
    ///
    /// // If new_size == Some(T) then T is the new size of the terminal.
    /// // If new_size == None then the terminal has not resized.
    /// let new_size = term.try_resize_with_cell(cell).unwrap();
    /// ```
    pub fn try_resize_with_cell(&mut self, cell: Cell) -> Result<Option<(usize, usize)>, Error> {
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize_with_cell(cell));
            return Ok(Some((self.cols, self.rows)));
        }
        Ok(None)
    }

    /// Sets the cursor position to (x, y).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::Terminal;
    ///
    /// let mut term = Terminal::new().unwrap();
    ///
    /// term.set_cursor(1, 1).unwrap();
    /// ```
    pub fn set_cursor(&mut self, x: usize, y: usize) -> Result<(), Error> {
        if self.cursor.pos().is_invalid() {
            try!(self.outbuffer.write_all(&self.driver.get(DevFn::ShowCursor)));
        }
        self.cursor.set_pos(Coordinate::Valid((x, y)));
        try!(self.send_cursor());
        Ok(())
    }

    /// Hides the cursor.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::Terminal;
    ///
    /// let mut term = Terminal::new().unwrap();
    ///
    /// term.hide_cursor().unwrap();
    /// ```
    pub fn hide_cursor(&mut self) -> Result<(), Error> {
        if self.cursor.pos().is_valid() {
            try!(self.outbuffer.write_all(&self.driver.get(DevFn::HideCursor)));
        }
        Ok(())
    }


    /// Gets an event from the event stream, waiting a maximum of `timeout_ms` milliseconds.
    ///
    /// Specifying a `timeout_ms` of -1 causes `get_event()` to block indefinitely, while
    /// specifying a `timeout_ms` of 0 causes `get_event()` to return immediately.
    ///
    /// Returns `Some(Event)` if an event was received within the specified timeout, or None
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::thread::sleep_ms;
    /// use rustty::{Terminal, Event};
    ///
    /// let mut term = Terminal::new().unwrap();
    ///
    /// let evt = term.get_event(1).unwrap();
    /// ```
    pub fn get_event(&mut self, timeout_ms: isize) -> Result<Option<Event>, Error> {
        // Check if the event buffer is empty.
        if self.eventbuffer.is_empty() {
            // Event buffer is empty, lets poll the terminal for events.
            let nevts = try!(self.read_events(timeout_ms));
            if nevts == 0 {
                // No events from the terminal either. Return none.
                Ok(None)
            } else {
                // Got at least one event from the terminal. Pop from the front of the event queue.
                Ok(self.eventbuffer.pop_front())
            }
        } else {
            // There is at least one event in the buffer already. Pop and return it.
            Ok(self.eventbuffer.pop_front())
        }
    }

    /// Prints a string at the specified position.
    ///
    /// This is a shorthand for setting each cell individually. `cell`'s style is going to be
    /// copied to each destination cell.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Terminal, Cell, Style, Color};
    ///
    /// let mut term = Terminal::new().unwrap();
    /// let cell = Cell::with_styles(Style::default(), Style::with_color(Color::Red));
    /// term.printline_with_cell(12, 42, "foobar", cell);
    /// ```
    pub fn printline_with_cell(&mut self, row: usize, col: usize, line: &str, cell: Cell) {
        let mut mutcell = cell;
        for (index, ch) in line.chars().enumerate() {
            let current_col = col + index;
            if current_col >= self.cols() {
                break;
            }
            mutcell.set_ch(ch);
            self[(row, current_col)] = mutcell;
        }
    }


    /// Prints a string at the specified position.
    ///
    /// Shorthand for `printline_with_cell(row, col line, Cell::default())`.
    pub fn printline(&mut self, row: usize, col: usize, line: &str) {
        self.printline_with_cell(row, col, line, Cell::default());
    }

    fn send_cursor(&mut self) -> Result<(), Error> {
        if let Coordinate::Valid((cx, cy)) = self.cursor.pos() {
            try!(self.outbuffer.write_all(&self.driver.get(DevFn::SetCursor(cx, cy))));
        }
        Ok(())
    }

    fn send_char(&mut self, coord: Coordinate<Pair>, ch: char) -> Result<(), Error> {
        self.cursor.set_pos(coord);
        if !self.cursor.is_seq() {
            try!(self.send_cursor());
        }
        try!(write!(self.outbuffer, "{}", ch));
        Ok(())
    }

    fn send_clear(&mut self, fg: Style, bg: Style) -> Result<(), Error> {
        try!(self.send_style(fg, bg));
        try!(self.outbuffer.write_all(&self.driver.get(DevFn::Clear)));
        try!(self.send_cursor());
        try!(self.flush());
        self.cursor.invalidate_last_pos();
        Ok(())
    }

    fn send_style(&mut self, fg: Style, bg: Style) -> Result<(), Error> {
        if fg != self.lastfg || bg != self.lastbg {
            try!(self.outbuffer.write_all(&self.driver.get(DevFn::Reset)));

            match fg.attr() {
                Attr::Bold => try!(self.outbuffer.write_all(&self.driver.get(DevFn::Bold))),
                Attr::Underline => try!(self.outbuffer.write_all(&self.driver.get(DevFn::Underline))),
                Attr::Reverse => try!(self.outbuffer.write_all(&self.driver.get(DevFn::Reverse))),
                _ => {},
            }

            match bg.attr() {
                Attr::Bold => try!(self.outbuffer.write_all(&self.driver.get(DevFn::Blink))),
                Attr::Underline => {},
                Attr::Reverse => try!(self.outbuffer.write_all(&self.driver.get(DevFn::Reverse))),
                _ => {},
            }

            try!(self.write_sgr(fg.color(), bg.color()));
            self.lastfg = fg;
            self.lastbg = bg;
        }
        Ok(())
    }

    fn write_sgr(&mut self, fgcol: Color, bgcol: Color) -> Result<(), Error> {
        match fgcol {
            Color::Default => {},
            fgc @ _ => {
                try!(self.outbuffer.write_all(&self.driver.get(DevFn::SetFg(fgc.as_byte()))));
            },
        }
        match bgcol {
            Color::Default => {},
            bgc @ _ => {
                try!(self.outbuffer.write_all(&self.driver.get(DevFn::SetBg(bgc.as_byte()))));
            },
        }
        Ok(())
    }

    fn resize(&mut self) -> Result<(), Error> {
        self.resize_with_cell(Cell::default())
    }

    /// Updates the size of the Terminal object to reflect that of the underlying terminal.
    fn resize_with_cell(&mut self, blank: Cell) -> Result<(), Error> {
        let mut ws = WindowSize::new();
        try!(unsafe {
            ioctl::read_into::<WindowSize>(self.rawtty, TIOCGWINSZ, &mut ws)
        });
        self.cols = ws.ws_col as usize;
        self.rows = ws.ws_row as usize;
        self.backbuffer.resize(self.cols, self.rows, blank);
        self.frontbuffer.resize(self.cols, self.rows, blank);
        self.frontbuffer.clear(blank);
        try!(self.send_clear(blank.fg(), blank.bg()));
        Ok(())
    }

    /// Attempts to read any available events from the terminal into the event buffer, waiting for
    /// the specified number of milliseconds for input to become available.
    ///
    /// Returns the number of events read into the buffer.
    fn read_events(&mut self, timeout_ms: isize) -> Result<usize, Error> {
        // Event vector to pass to kernel.
        let mut events: Vec<EpollEvent> = Vec::new();
        events.push(EpollEvent { events: EpollEventKind::empty(), data: 0 });

        let mut nepolls: usize;
        // Because the sigwinch handler will interrupt epoll, if epoll returns EINTR we loop
        // and try again. All other errors will return normally.
        loop {
            nepolls = match epoll_wait(self.epfd, &mut events, timeout_ms) {
                Ok(n) => n,
                Err(e) if e.errno() == Errno::EINTR => {
                    // Errno is EINTR, loop and try again.
                    continue;
                },
                Err(e) => {
                    // Error that isn't EINTR, return up the stack.
                    return Err(Error::from(e));
                },
            };
            // We will only reach this point if epoll_wait succeeds, therefore we have assigned
            // to nevents and can break.
            break;
        }

        if nepolls == 0 {
            // No input available. Return None.
            Ok(0)
        } else {
            // Input is available from the terminal.
            // Get an iterator of chars over the input stream.
            let mut buf = String::new();
            try!(self.tty.read_to_string(&mut buf));
            let mut n = 0;
            for ch in buf.chars() {
                // Push each character onto the event queue and increment the count.
                self.eventbuffer.push_back(Event::Key(ch));
                n += 1;
            }
            Ok(n)
        }
    }

    fn flush(&mut self) -> Result<(), Error> {
        try!(self.tty.write_all(&self.outbuffer));
        self.outbuffer.clear();
        Ok(())
    }
}

impl Index<usize> for Terminal {
    type Output = Vec<Cell>;

    fn index(&self, index: usize) -> &Vec<Cell> {
        &self.backbuffer[index]
    }
}

impl Index<(usize, usize)> for Terminal {
    type Output = Cell;

    fn index<'a>(&'a self, index: (usize, usize)) -> &'a Cell {
        &self.backbuffer[index]
    }
}

impl IndexMut<usize> for Terminal {
    fn index_mut(&mut self, index: usize) -> &mut Vec<Cell> {
        &mut self.backbuffer[index]
    }
}

impl IndexMut<(usize, usize)> for Terminal {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Cell {
        &mut self.backbuffer[index]
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.outbuffer.write_all(&self.driver.get(DevFn::ShowCursor)).unwrap();
        self.outbuffer.write_all(&self.driver.get(DevFn::Reset)).unwrap();
        self.outbuffer.write_all(&self.driver.get(DevFn::Clear)).unwrap();
        self.outbuffer.write_all(&self.driver.get(DevFn::ExitCa)).unwrap();
        self.flush().unwrap();
        termios::tcsetattr(self.rawtty, SetArg::TCSAFLUSH, &self.orig_tios).unwrap();
        SIGWINCH_STATUS.store(false, Ordering::SeqCst);
        RUSTTY_STATUS.store(false, Ordering::SeqCst);
    }
}

// Sigwinch handler to notify when window has resized.
extern fn sigwinch_handler(_: i32) {
    SIGWINCH_STATUS.store(true, Ordering::SeqCst);
}

// Window size struct to pass to the kernel.
#[repr(C)]
struct WindowSize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}

impl WindowSize {
    fn new() -> WindowSize {
        WindowSize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        }
    }
}

