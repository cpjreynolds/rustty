use std::ops::{Index, IndexMut};
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::fs::File;
use std::os::unix::io::{AsRawFd, RawFd};
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

use nix::sys::termios;
use nix::sys::termios::{IGNBRK, BRKINT, PARMRK, ISTRIP, INLCR, IGNCR, ICRNL, IXON};
use nix::sys::termios::{OPOST, ECHO, ECHONL, ICANON, ISIG, IEXTEN, CSIZE, PARENB, CS8};
use nix::sys::termios::{VMIN, VTIME};
use nix::sys::termios::SetArg;
use nix::sys::signal;
use nix::sys::signal::{SockFlag, SigSet};
use nix::sys::signal::signal::SIGWINCH;
use nix::sys::ioctl;

use util::error::Error;
use core::device::{Device, DevFunc};
use core::cellbuffer::{CellBuffer, Cell, Style, Color, Attr};
use core::bytebuffer::ByteBuffer;
use core::cursor::Cursor;

/// Set to true by the sigwinch handler. Reset to false when buffers are resized.
static SIGWINCH_STATUS: AtomicBool = ATOMIC_BOOL_INIT;

/// Ensures that there is only ever one Terminal object at any one time.
/// Set to true on creation of a Terminal object.
/// Reset to false when terminal object goes out of scope.
static RUSTTY_STATUS: AtomicBool = ATOMIC_BOOL_INIT;

/// Constant for TIOCGWINSZ syscall on OS X.
#[cfg(target_os="macos")]
const TIOCGWINSZ: u64 = 0x40087468;

/// Constant for TIOCGWINSZ syscall on Linux.
#[cfg(target_os="linux")]
const TIOCGWINSZ: u64 = 0x00005413;


/// A representation of the current terminal window.
///
/// Only one `Terminal` object can exist at any one time.
/// When a `Terminal` goes out of scope it resets the underlying terminal to its original state.
pub struct Terminal {
    orig_tios: termios::Termios, // Original underlying terminal state.
    tty: File, // Underlying terminal file.
    rawtty: RawFd, // Raw file descriptor of underlying terminal file.
    cols: usize, // Number of columns in the terminal window.
    rows: usize, // Number of rows in the terminal window.
    device: &'static Device, // Underlying terminal device (xterm, gnome, etc.).
    backbuffer: CellBuffer, // Internal backbuffer.
    frontbuffer: CellBuffer, // Internal frontbuffer.
    outbuffer: ByteBuffer, // Internal output buffer.
    inbuffer: ByteBuffer, // Internal input buffer.
    lastfg: Style, // Last foreground style written to the output buffer.
    lastbg: Style, // Last background style written to the input buffer.
    cursor: Cursor, // Current cursor position.
    cursor_last: Cursor, // Last cursor position.
}

impl Terminal {
    /// Creates a new `Terminal` with default settings.
    pub fn new() -> Result<Terminal, Error> {
        Terminal::with_cell(Cell::default())
    }

    /// Creates a new `Terminal` with each cell set to the given character.
    pub fn with_char(ch: char) -> Result<Terminal, Error> {
        Terminal::with_cell(Cell::with_char(ch))
    }

    /// Creates a new `Terminal` with each cell set to the given styles.
    pub fn with_styles(fg: Style, bg: Style) -> Result<Terminal, Error> {
        Terminal::with_cell(Cell::with_styles(fg, bg))
    }

    /// Creates a new `Terminal` with each cell set to the given cell.
    pub fn with_cell(cell: Cell) -> Result<Terminal, Error> {
        // Make sure there is only ever one instance.
        if RUSTTY_STATUS.compare_and_swap(false, true, Ordering::SeqCst) {
            return Err(Error::new("Rustty already initialized"))
        }

        // Return the device for the user's terminal.
        let device = try!(Device::new());

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

        // Create the terminal object to hold all of our required state.
        let mut terminal = Terminal {
            orig_tios: orig_tios,
            tty: tty,
            rawtty: rawtty,
            cols: 0,
            rows: 0,
            device: device,
            backbuffer: CellBuffer::with_cell(0, 0, cell),
            frontbuffer: CellBuffer::with_cell(0, 0, cell),
            outbuffer: ByteBuffer::with_capacity(32 * 1024),
            inbuffer: ByteBuffer::with_capacity(128),
            lastfg: cell.fg(),
            lastbg: cell.bg(),
            cursor: Cursor::Invalid,
            cursor_last: Cursor::Invalid,
        };

        // Switch to alternate screen buffer. Writes the control code to the output buffer.
        try!(write!(terminal.outbuffer, "{}", &terminal.device[DevFunc::EnterCa]));

        // Enter keypad. Writes the control code to the output buffer.
        try!(write!(terminal.outbuffer, "{}", &terminal.device[DevFunc::EnterKeypad]));

        // Hide cursor. Writes the control code to the output buffer.
        try!(write!(terminal.outbuffer, "{}", &terminal.device[DevFunc::HideCursor]));

        try!(terminal.resize_with_cell(cell));

        // Return the initialized terminal object.
        Ok(terminal)
    }

    /// Send the current backbuffer to be displayed.
    pub fn swap_buffers(&mut self) -> Result<(), Error> {
        // Check whether the window has been resized; if it has then update and resize the buffers.
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize());
        }

        // Invalidate the last cursor position.
        self.cursor_last = Cursor::Invalid;

        for x in 0..self.cols() {
            if self.backbuffer[x] == self.frontbuffer[x] {
                continue; // Redundant draw checking.
            }
            for y in 0..self.rows() {
                if self.backbuffer[x][y] == self.frontbuffer[x][y] {
                    continue; // Don't make rendundant draws.
                } else {
                    self.frontbuffer[x][y] = self.backbuffer[x][y];
                }
                let cell = self.backbuffer[x][y];
                try!(self.send_style(cell.fg(), cell.bg()));
                try!(self.send_char(Cursor::Valid(x, y), cell.ch()));
            }
        }

        if self.cursor != Cursor::Invalid {
            try!(self.send_current_cursor());
        }
        try!(self.flush());
        Ok(())
    }

    /// Returns the width of the terminal.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns the height of the terminal.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the size of the terminal as (cols, rows).
    pub fn size(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }

    /// Clears the internal backbuffer with whitespace.
    pub fn clear(&mut self) -> Result<(), Error> {
        // Check whether the window has been resized; if it has then update and resize the buffers.
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize());
        }
        self.backbuffer.clear();
        Ok(())
    }

    /// Clears the internal backbuffer with the given character.
    pub fn clear_with_char(&mut self, ch: char) -> Result<(), Error> {
        // Check whether the window has been resized; if it has then update and resize the buffers.
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize());
        }
        self.backbuffer.clear_with_char(ch);
        Ok(())
    }

    /// Clears the internal backbuffer with the given styles.
    pub fn clear_with_styles(&mut self, fg: Style, bg: Style) -> Result<(), Error> {
        // Check whether the window has been resized; if it has then update and resize the buffers.
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize());
        }
        self.backbuffer.clear_with_styles(fg, bg);
        Ok(())
    }

    /// Clears the internal backbuffer with the given cell.
    pub fn clear_with_cell(&mut self, cell: Cell) -> Result<(), Error> {
        // Check whether the window has been resized; if it has then update and resize the buffers.
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize());
        }
        self.backbuffer.clear_with_cell(cell);
        Ok(())
    }

    /// Checks whether the window size has changed, returning `true` if it has.
    pub fn check_resize(&self) -> bool {
        SIGWINCH_STATUS.load(Ordering::SeqCst)
    }

    pub fn try_resize(&mut self) -> Result<Option<(usize, usize)>, Error> {
        self.try_resize_with_cell(Cell::default())
    }

    pub fn try_resize_with_char(&mut self, ch: char) -> Result<Option<(usize, usize)>, Error> {
        self.try_resize_with_cell(Cell::with_char(ch))
    }

    pub fn try_resize_with_styles(&mut self,
                                  fg: Style,
                                  bg: Style) -> Result<Option<(usize, usize)>, Error> {
        self.try_resize_with_cell(Cell::with_styles(fg, bg))
    }

    /// Performs a resize if the window size has changed, returning the new size. Otherwise returns
    /// none.
    pub fn try_resize_with_cell(&mut self, cell: Cell) -> Result<Option<(usize, usize)>, Error> {
        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.resize_with_cell(cell));
            return Ok(Some((self.cols, self.rows)));
        }
        Ok(None)
    }

    /// Sets the cursor position.
    pub fn set_cursor(&mut self, c: Cursor) -> Result<(), Error> {
        if self.cursor == Cursor::Invalid && c != Cursor::Invalid {
            try!(write!(self.outbuffer, "{}", &self.device[DevFunc::ShowCursor]));
        }

        if self.cursor != Cursor::Invalid && c == Cursor::Invalid {
            try!(write!(self.outbuffer, "{}", &self.device[DevFunc::HideCursor]));
        }

        self.cursor = c;

        if self.cursor != Cursor::Invalid {
            try!(self.send_cursor(c));
        }
        Ok(())
    }

    fn send_cursor(&mut self, c: Cursor) -> Result<(), Error> {
        if let Cursor::Valid(cx, cy) = c {
            try!(write!(self.outbuffer, "\x1b[{};{}H", cy+1, cx+1));
        } else {
            try!(write!(self.outbuffer, "\x1b[{};{}H", 0, 0));
        }
        Ok(())
    }

    fn send_current_cursor(&mut self) -> Result<(), Error> {
        if let Cursor::Valid(cx, cy) = self.cursor {
            try!(write!(self.outbuffer, "\x1b[{};{}H", cy+1, cx+1));
        } else {
            try!(write!(self.outbuffer, "\x1b[{};{}H", 0, 0));
        }
        Ok(())
    }

    fn send_char(&mut self, cursor: Cursor, ch: char) -> Result<(), Error> {
        if let Cursor::Valid(cx, cy) = cursor {
            if let Cursor::Valid(lx, ly) = self.cursor_last {
                if (cx, cy) != (lx + 1, ly) {
                    try!(self.send_cursor(cursor));
                }
            }
        }
        self.cursor_last = cursor;
        try!(write!(self.outbuffer, "{}", ch));
        Ok(())
    }

    fn send_clear(&mut self, fg: Style, bg: Style) -> Result<(), Error> {
        try!(self.send_style(fg, bg));
        try!(write!(self.outbuffer, "{}", &self.device[DevFunc::ClearScreen]));
        if self.cursor != Cursor::Invalid {
            try!(self.send_current_cursor());
        }
        try!(self.flush());
        self.cursor_last = Cursor::Invalid;
        Ok(())
    }

    fn send_style(&mut self, fg: Style, bg: Style) -> Result<(), Error> {
        if fg != self.lastfg || bg != self.lastbg {
            try!(write!(self.outbuffer, "{}", &self.device[DevFunc::Sgr0]));

            match fg.attr() {
                Attr::Bold => try!(write!(self.outbuffer, "{}", &self.device[DevFunc::Bold])),
                Attr::Underline => try!(write!(self.outbuffer, "{}", &self.device[DevFunc::Underline])),
                Attr::Reverse => try!(write!(self.outbuffer, "{}", &self.device[DevFunc::Reverse])),
                _ => {},
            }

            match bg.attr() {
                Attr::Bold => try!(write!(self.outbuffer, "{}", &self.device[DevFunc::Blink])),
                Attr::Underline => {},
                Attr::Reverse => try!(write!(self.outbuffer, "{}", &self.device[DevFunc::Reverse])),
                _ => {},
            }

            if fg.color() != Color::Default {
                if bg.color() != Color::Default {
                    try!(self.write_sgr(fg.color(), bg.color()))
                } else {
                    try!(self.write_sgr_fg(fg.color()))
                }
            } else if bg.color() != Color::Default {
                try!(self.write_sgr_bg(bg.color()))
            }
            self.lastfg = fg;
            self.lastbg = bg;
        }
        Ok(())
    }

    fn write_sgr_fg(&mut self, fgcol: Color) -> Result<(), Error> {
        try!(write!(self.outbuffer, "\x1b[3{}m", fgcol as usize));
        Ok(())
    }

    fn write_sgr_bg(&mut self, bgcol: Color) -> Result<(), Error> {
        try!(write!(self.outbuffer, "\x1b[4{}m", bgcol as usize));
        Ok(())
    }

    fn write_sgr(&mut self, fgcol: Color, bgcol: Color) -> Result<(), Error> {
        try!(write!(self.outbuffer, "\x1b[3{};4{}m", fgcol as usize, bgcol as usize));
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
        try!(self.send_clear(blank.fg(), blank.bg()));
        Ok(())
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

impl IndexMut<usize> for Terminal {
    fn index_mut(&mut self, index: usize) -> &mut Vec<Cell> {
        &mut self.backbuffer[index]
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        write!(self.outbuffer, "{}", &self.device[DevFunc::ShowCursor]).unwrap();
        write!(self.outbuffer, "{}", &self.device[DevFunc::Sgr0]).unwrap();
        write!(self.outbuffer, "{}", &self.device[DevFunc::ClearScreen]).unwrap();
        write!(self.outbuffer, "{}", &self.device[DevFunc::ExitCa]).unwrap();
        write!(self.outbuffer, "{}", &self.device[DevFunc::ExitKeypad]).unwrap();
        write!(self.outbuffer, "{}", &self.device[DevFunc::ExitMouse]).unwrap();
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

