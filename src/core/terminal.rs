use std::ops::{Index, IndexMut, Deref, DerefMut};
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::fs::OpenOptions;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;
use std::ptr;
use std::mem;

use libc;

use gag::BufferRedirect;

use core::cellbuffer::{CellBuffer, Cell, Color, Attr};
use core::input::Event;
use core::driver::{DevFn, Driver};

/// Set to true by the sigwinch handler. Reset to false when buffers are resized.
static SIGWINCH_STATUS: AtomicBool = ATOMIC_BOOL_INIT;

/// Ensures that there is only ever one Terminal object at any one time.
/// Set to true on creation of a Terminal object.
/// Reset to false when terminal object goes out of scope.
static RUSTTY_STATUS: AtomicBool = ATOMIC_BOOL_INIT;

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
/// use rustty::{Terminal, Cell, Color};
///
/// // Construct a new Terminal.
/// let mut term = Terminal::new().unwrap();
///
/// // Terminals can be indexed to access specific cells.
/// // Indices are by column then row, corresponding to a cell's x and y coordinates.
/// term[(0, 0)] = Cell::with_char('x');
/// assert_eq!(term[(0, 0)].ch(), 'x');
///
/// term[(0, 1)].set_bg(Color::Red);
/// assert_eq!(term[(0, 1)].bg(), Color::Red);
///
/// term[(0, 2)].set_fg(Color::Blue);
/// assert_eq!(term[(0, 2)].fg(), Color::Blue);
/// ```
pub struct Terminal {
    // Underlying terminal file.
    tty: File,
    // Number of columns in the terminal window.
    cols: usize,
    // Number of rows in the terminal window.
    rows: usize,
    // Terminal driver (terminfo).
    driver: Driver,
    // Internal backbuffer.
    backbuffer: CellBuffer,
    // Internal frontbuffer.
    frontbuffer: CellBuffer,
    // Internal output buffer.
    outbuffer: OutBuffer,
    // Event buffer.
    eventbuffer: EventBuffer,
    // Stderr handle to dump on panics.
    stderr_handle: BufferRedirect,
    // Original termios structure.
    orig_tios: libc::termios,
}

impl Terminal {
    /// Constructs a new `Terminal`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustty::Terminal;
    ///
    /// let mut term = Terminal::new().unwrap();
    /// ```
    pub fn new() -> Result<Terminal, Error> {
        // Make sure there is only ever one instance.
        if RUSTTY_STATUS.compare_and_swap(false, true, Ordering::SeqCst) {
            return Err(Error::new(ErrorKind::AlreadyExists, "terminal already initialized"));
        }

        // Create the terminal driver.
        // If this returns an error then the terminal is not supported.
        let driver = try!(Driver::new());

        // Open the terminal file for the controlling process.
        let tty = try!(OpenOptions::new()
            .write(true)
            .read(true)
            .open("/dev/tty"));

        // Set up the signal handler for SIGWINCH, which will notify us when the window size has
        // changed; it does this by atomically setting SIGWINCH_STATUS to 'true'.
        let handler = sigwinch_handler as libc::size_t;
        let mut sa_winch: libc::sigaction = unsafe { mem::zeroed() };
        sa_winch.sa_sigaction = handler;
        let res = unsafe { libc::sigaction(libc::SIGWINCH, &sa_winch, ptr::null_mut()) };
        if res != 0 {
            return Err(Error::last_os_error());
        }

        // Create and initialize the original termios struct, so we can reset it on drop.
        let mut orig_tios = unsafe { mem::uninitialized() };
        let res = unsafe { libc::tcgetattr(tty.as_raw_fd(), &mut orig_tios) };
        if res != 0 {
            return Err(Error::last_os_error());
        }

        let mut terminal = Terminal {
            tty: tty,
            cols: 0,
            rows: 0,
            driver: driver,
            backbuffer: CellBuffer::new(0, 0),
            frontbuffer: CellBuffer::new(0, 0),
            outbuffer: OutBuffer::with_capacity(32 * 1024),
            eventbuffer: EventBuffer::with_capacity(128),
            stderr_handle: BufferRedirect::stderr().unwrap(),
            orig_tios: orig_tios,
        };

        // Set the termios options we need.
        try!(terminal.set_termios());

        // Switch to alternate screen buffer. Writes the control code to the output buffer.
        try!(terminal.outbuffer.write_all(&terminal.driver.get(DevFn::EnterCa)));

        // Hide cursor. Writes the control code to the output buffer.
        try!(terminal.outbuffer.write_all(&terminal.driver.get(DevFn::HideCursor)));

        // Resize the buffers to the size of the underlying terminal.
        try!(terminal.resize());

        // Return the initialized `Terminal`.
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
    /// term.refresh().unwrap();
    /// ```
    pub fn refresh(&mut self) -> Result<(), Error> {
        // Check whether the window has been resized; if it has then update and resize the buffers.
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize());
        }

        for y in 0..self.rows() {
            for x in 0..self.cols() {
                if self.frontbuffer[(x, y)] == self.backbuffer[(x, y)] {
                    continue; // Don't redraw cells that haven't changed.
                } else {
                    let cell = self.backbuffer[(x, y)];
                    try!(self.send_style(cell));
                    try!(self.send_char(x, y, cell.ch()));
                    // Update the frontbuffer to reflect the changes.
                    self.frontbuffer[(x, y)] = cell;
                }
            }
        }
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
    /// assert_eq!(term[(0, 0)].ch(), 'x');
    ///
    /// term.clear(cell2).unwrap();
    /// assert_eq!(term[(0, 0)].ch(), 'y');
    /// ```
    pub fn clear(&mut self, cell: Cell) -> Result<(), Error> {
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
    /// This method will be called automatically on each call to `refresh()` or a `clear()`
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
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize());
            return Ok(Some((self.cols, self.rows)));
        }
        Ok(None)
    }

    /// Gets an event from the event stream, waiting at most the value specified in `timeout`.
    ///
    /// Specifying a `timeout` of `None` causes `get_event()` to block indefinitely, while
    /// specifying a `timeout` of zero causes `get_event()` to return immediately.
    ///
    /// Returns `Some(Event)` if an event was received within the specified timeout, or None
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::thread::sleep_ms;
    /// use rustty::{Terminal, Event};
    /// use std::time::Duration;
    ///
    /// let mut term = Terminal::new().unwrap();
    ///
    /// let evt = term.get_event(Duration::from_secs(1)).unwrap();
    /// ```
    pub fn get_event(&mut self, timeout: Duration) -> Result<Option<Event>, Error> {
        // Check if the event buffer is empty.
        if self.eventbuffer.is_empty() {
            // Event buffer is empty, lets poll the terminal for events.
            let nevts = try!(self.read_events(timeout));
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

    fn send_cursor(&mut self, x: usize, y: usize) -> Result<(), Error> {
        try!(self.outbuffer.write_all(&self.driver.get(DevFn::SetCursor(x, y))));
        Ok(())
    }

    // Sets the cursor to the specified coordinates and then writes the specified character.
    //
    // At the moment, wide characters are going to make things go very, very wrong...probably.
    fn send_char(&mut self, x: usize, y: usize, ch: char) -> Result<(), Error> {
        try!(self.send_cursor(x, y));
        try!(write!(self.outbuffer, "{}", ch));
        Ok(())
    }

    // Clears the terminal with the default style.
    fn send_clear(&mut self) -> Result<(), Error> {
        try!(self.outbuffer.write_all(&self.driver.get(DevFn::Reset)));
        try!(self.outbuffer.write_all(&self.driver.get(DevFn::Clear)));
        try!(self.flush());
        Ok(())
    }

    fn send_style(&mut self, cell: Cell) -> Result<(), Error> {
        try!(self.outbuffer.write_all(&self.driver.get(DevFn::Reset)));

        match cell.attrs() {
            Attr::Bold => try!(self.outbuffer.write_all(&self.driver.get(DevFn::Bold))),
            Attr::Underline => try!(self.outbuffer.write_all(&self.driver.get(DevFn::Underline))),
            Attr::Reverse => try!(self.outbuffer.write_all(&self.driver.get(DevFn::Reverse))),
            _ => {}
        }

        try!(self.write_sgr(cell.fg(), cell.bg()));
        Ok(())
    }

    fn write_sgr(&mut self, fgcol: Color, bgcol: Color) -> Result<(), Error> {
        match fgcol {
            Color::Default => {}
            fgc @ _ => {
                try!(self.outbuffer.write_all(&self.driver.get(DevFn::SetFg(fgc.as_byte()))));
            }
        }
        match bgcol {
            Color::Default => {}
            bgc @ _ => {
                try!(self.outbuffer.write_all(&self.driver.get(DevFn::SetBg(bgc.as_byte()))));
            }
        }
        Ok(())
    }

    /// Updates the size of the Terminal object to reflect that of the underlying terminal.
    fn resize(&mut self) -> Result<(), Error> {
        let (cols, rows) = try!(self.window_size());
        self.cols = cols;
        self.rows = rows;
        self.backbuffer.resize(self.cols, self.rows, Cell::default());
        self.frontbuffer.resize(self.cols, self.rows, Cell::default());
        self.frontbuffer.clear(Cell::default());
        try!(self.send_clear());
        Ok(())
    }

    /// Attempts to read any available events from the terminal into the event buffer, waiting for
    /// the specified timeout for input to become available.
    ///
    /// Returns the number of events read into the buffer.
    fn read_events(&mut self, timeout: Duration) -> Result<usize, Error> {
        let nevts;
        let mut timeout = libc::timeval {
            tv_sec: timeout.as_secs() as libc::time_t,
            tv_usec: (timeout.subsec_nanos() as libc::suseconds_t) / 1000,
        };
        let rawfd = self.tty.as_raw_fd();
        let nfds = rawfd + 1;

        let mut rfds: libc::fd_set = unsafe { mem::zeroed() };
        unsafe {
            libc::FD_SET(rawfd, &mut rfds);
        }

        // Because the sigwinch handler will interrupt select, if select returns EINTR we loop
        // and try again. All other errors will return normally.
        loop {
            let res = unsafe {
                libc::select(nfds,
                             &mut rfds,
                             ptr::null_mut(),
                             ptr::null_mut(),
                             &mut timeout)
            };

            if res == -1 {
                let err = Error::last_os_error();

                if err.kind() == ErrorKind::Interrupted {
                    // Errno is EINTR, loop and try again.
                    continue;
                } else {
                    // Error other than EINTR, return to caller.
                    return Err(err);
                }
            } else {
                nevts = res;
                break;
            }
        }

        if nevts == 0 {
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
        if thread::panicking() {
            let mut error = String::new();
            self.stderr_handle.read_to_string(&mut error).unwrap();
            print!("{}", error);
        }
        Ok(())
    }

    fn set_termios(&self) -> Result<(), Error> {
        let fd = self.tty.as_raw_fd();
        let mut tios = self.orig_tios.clone();

        tios.c_iflag &= !(libc::IGNBRK | libc::BRKINT | libc::PARMRK | libc::ISTRIP |
                          libc::INLCR | libc::IGNCR | libc::ICRNL |
                          libc::IXON);
        tios.c_oflag &= !libc::OPOST;
        tios.c_lflag &= !(libc::ECHO | libc::ECHONL | libc::ICANON | libc::ISIG | libc::IEXTEN);
        tios.c_cflag &= !(libc::CSIZE | libc::PARENB);
        tios.c_cflag |= libc::CS8;
        tios.c_cc[libc::VMIN] = 0;
        tios.c_cc[libc::VTIME] = 0;

        let res = unsafe { libc::tcsetattr(fd, libc::TCSAFLUSH, &tios) };

        if res != 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn window_size(&self) -> Result<(usize, usize), Error> {
        let fd = self.tty.as_raw_fd();
        let mut ws: libc::winsize = unsafe { mem::uninitialized() };

        let res = unsafe { libc::ioctl(fd, libc::TIOCGWINSZ, &mut ws) };
        if res != 0 {
            Err(Error::last_os_error())
        } else {
            Ok((ws.ws_col as usize, ws.ws_row as usize))
        }
    }

    pub fn reset_termios(&self) -> Result<(), Error> {
        let fd = self.tty.as_raw_fd();
        let res = unsafe { libc::tcsetattr(fd, libc::TCSAFLUSH, &self.orig_tios) };
        if res != 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

impl Deref for Terminal {
    type Target = [Cell];

    fn deref<'a>(&'a self) -> &'a [Cell] {
        &self.backbuffer
    }
}

impl DerefMut for Terminal {
    fn deref_mut<'a>(&'a mut self) -> &'a mut [Cell] {
        &mut self.backbuffer
    }
}

impl Index<(usize, usize)> for Terminal {
    type Output = Cell;

    fn index<'a>(&'a self, index: (usize, usize)) -> &'a Cell {
        &self.backbuffer[index]
    }
}

impl IndexMut<(usize, usize)> for Terminal {
    fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut Cell {
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
        self.reset_termios().unwrap();
        SIGWINCH_STATUS.store(false, Ordering::SeqCst);
        RUSTTY_STATUS.store(false, Ordering::SeqCst);
    }
}

// Sigwinch handler to notify when window has resized.
extern "C" fn sigwinch_handler(_: i32) {
    SIGWINCH_STATUS.store(true, Ordering::SeqCst);
}
