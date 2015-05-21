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

use Device;
use DevFunc;
use TtyError;
use CellBuffer;
use ByteBuffer;
use Cell;
use Color;
use Style;
use Attr;

/// Set to true by the sigwinch handler. Reset to false when handled elsewhere.
static SIGWINCH_STATUS: AtomicBool = ATOMIC_BOOL_INIT;

/// Set to true when there is an active Terminal instance.
/// Reset to false when it goes out of scope.
static RUSTTY_STATUS: AtomicBool = ATOMIC_BOOL_INIT;

#[cfg(target_os="macos")]
const TIOCGWINSZ: u64 = 0x40087468;

#[cfg(target_os="linux")]
const TIOCGWINSZ: u64 = 0x00005413;

macro_rules! is_cursor_hidden {
    ( $x:expr ) => {
        $x == Cursor::Invalid
    };
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Cursor {
    Valid(usize, usize),
    Invalid,
}

pub struct Terminal {
    orig_tios: termios::Termios,
    tios: termios::Termios,
    tty: File,
    rawtty: RawFd,
    cols: usize,
    rows: usize,
    device: &'static Device,
    backbuffer: CellBuffer,
    frontbuffer: CellBuffer,
    outbuffer: ByteBuffer,
    foreground: Style,
    background: Style,
    cursor: Cursor,
    cursor_last: Cursor,
}

impl Terminal {
    pub fn new() -> Result<Terminal, TtyError> {
        // Make sure there is only ever one instance.
        if RUSTTY_STATUS.compare_and_swap(false, true, Ordering::SeqCst) {
            return Err(TtyError::new("Rustty already initialized"))
        }

        // Return the device object for the user's terminal.
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
        unsafe {
            if let Err(e) = signal::sigaction(SIGWINCH, &sa_winch) {
                panic!("{:?}", e);
            }
        }

        // Get the original state of the terminal so we can restore it on drop.
        let orig_tios = termios::tcgetattr(rawtty).unwrap();

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

        // Make the system call to change terminal parameters. Panic if this fails.
        // FIXME: Better error handling.
        termios::tcsetattr(rawtty, SetArg::TCSAFLUSH, &tios).unwrap();

        // Create the terminal object to hold all of our required state.
        let mut terminal = Terminal {
            orig_tios: orig_tios,
            tios: tios,
            tty: tty,
            rawtty: rawtty,
            cols: 0,
            rows: 0,
            device: device,
            backbuffer: CellBuffer::new(0, 0),
            frontbuffer: CellBuffer::new(0, 0),
            outbuffer: ByteBuffer::with_capacity(32 * 1024),
            foreground: Style::default(),
            background: Style::default(),
            cursor: Cursor::Invalid,
            cursor_last: Cursor::Invalid,
        };

        // Switch to alternate screen buffer. Writes the control code to the output buffer.
        try!(terminal.outbuffer.write_all(&terminal.device[DevFunc::EnterCa]));

        // Enter keypad. Writes the control code to the output buffer.
        try!(terminal.outbuffer.write_all(&terminal.device[DevFunc::EnterKeypad]));

        // Hide cursor. Writes the control code to the output buffer.
        try!(terminal.outbuffer.write_all(&terminal.device[DevFunc::HideCursor]));

        // Clear screen. Writes the control code to the output buffer.
        try!(terminal.send_clear(Cell::blank_default()));

        // Updates the terminal object's size. Doesn't resize anything.
        try!(terminal.update_size());

        // Resize the backbuffer to reflect the updated size. Use the default cell for
        // blank space.
        try!(terminal.resize(Cell::blank_default()));

        // Clear the back buffer with the default cell.
        terminal.clear_backbuffer(Cell::blank_default());
        terminal.clear_frontbuffer(Cell::blank_default());

        // Return the initialized terminal object.
        Ok(terminal)
    }

    pub fn swap(&mut self) -> Result<(), TtyError> {
        self.cursor_last = Cursor::Invalid;

        if SIGWINCH_STATUS.compare_and_swap(true, false, Ordering::SeqCst) {
            try!(self.update_size());
            try!(self.resize(Cell::blank_default()));
        }

        for y in 0..self.rows() {
            for x in 0..self.cols() {
                if self.backbuffer.cells[y][x] == self.frontbuffer.cells[y][x] {
                    continue;
                } else {
                    self.frontbuffer.cells[y][x] = self.backbuffer.cells[y][x];
                }
                let Cell { ch, fg, bg } = self.backbuffer.cells[y][x];
                try!(self.send_style(fg, bg));
                try!(self.send_char(x, y, ch));
            }
        }

        if self.cursor != Cursor::Invalid {
            try!(self.write_current_cursor());
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

    /// Clears the internal back buffer with the provided cell.
    pub fn clear(&mut self, cell: Cell) {
        self.clear_backbuffer(cell);
    }

    /// Clears the internal back buffer with the provided cell.
    fn clear_backbuffer(&mut self, cell: Cell) {
        self.backbuffer.clear(Cell::blank(cell.fg, cell.bg));
    }

    fn clear_frontbuffer(&mut self, cell: Cell) {
        self.frontbuffer.clear(Cell::blank(cell.fg, cell.bg));
    }

    /// Sets the cursor position.
    fn set_cursor(&mut self, c: Cursor) -> Result<(), TtyError> {
        if self.cursor == Cursor::Invalid && c != Cursor::Invalid {
            try!(self.outbuffer.write_all(&self.device[DevFunc::ShowCursor]));
        }

        if self.cursor != Cursor::Invalid && c == Cursor::Invalid {
            try!(self.outbuffer.write_all(&self.device[DevFunc::HideCursor]));
        }

        self.cursor = c;

        if self.cursor == Cursor::Invalid {
            try!(self.write_cursor(c));
        }
        Ok(())
    }

    fn write_cursor(&mut self, c: Cursor) -> Result<(), TtyError> {
        if let Cursor::Valid(cx, cy) = c {
            try!(write!(self.outbuffer, "\x1b[{};{}H", cy+1, cx+1));
        } else {
            try!(write!(self.outbuffer, "\x1b[{};{}H", 0, 0));
        }
        Ok(())
    }

    fn write_current_cursor(&mut self) -> Result<(), TtyError> {
        if let Cursor::Valid(cx, cy) = self.cursor {
            try!(write!(self.outbuffer, "\x1b[{};{}H", cy+1, cx+1));
        } else {
            try!(write!(self.outbuffer, "\x1b[{};{}H", 0, 0));
        }
        Ok(())
    }

    fn send_clear(&mut self, cell: Cell) -> Result<(), TtyError> {
        try!(self.send_style(cell.fg, cell.bg));
        try!(self.outbuffer.write_all(&self.device[DevFunc::ClearScreen]));
        if self.cursor != Cursor::Invalid {
            try!(self.write_current_cursor());
        }
        try!(self.flush());
        self.cursor_last = Cursor::Invalid;
        Ok(())
    }

    fn send_style(&mut self, fg: Style, bg: Style) -> Result<(), TtyError> {
        try!(self.outbuffer.write_all(&self.device[DevFunc::Sgr0]));
        let Style(fgcol, fgattr) = fg;
        let Style(bgcol, bgattr) = bg;

        match fgattr {
            Attr::Bold => try!(self.outbuffer.write_all(&self.device[DevFunc::Bold])),
            Attr::Underline => try!(self.outbuffer.write_all(&self.device[DevFunc::Underline])),
            Attr::Reverse => try!(self.outbuffer.write_all(&self.device[DevFunc::Reverse])),
            _ => {},
        }

        match bgattr {
            Attr::Bold => try!(self.outbuffer.write_all(&self.device[DevFunc::Blink])),
            Attr::Underline => {},
            Attr::Reverse => try!(self.outbuffer.write_all(&self.device[DevFunc::Reverse])),
            _ => {},
        }

        if fgcol != Color::Default {
            if bgcol != Color::Default {
                try!(self.write_sgr(fgcol, bgcol))
            } else {
                try!(self.write_sgr_fg(fgcol))
            }
        } else if bgcol != Color::Default {
            try!(self.write_sgr_bg(bgcol))
        }
        Ok(())
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if x >= self.backbuffer.cols() {
            return;
        }
        if y >= self.backbuffer.rows() {
            return;
        }
        self.backbuffer.cells[y][x] = cell;
    }

    fn write_sgr_fg(&mut self, fgcol: Color) -> Result<(), TtyError> {
        try!(write!(self.outbuffer, "\x1b[3{}m", (fgcol as usize) - 1));
        Ok(())
    }

    fn write_sgr_bg(&mut self, bgcol: Color) -> Result<(), TtyError> {
        try!(write!(self.outbuffer, "\x1b[4{}m", (bgcol as usize) - 1));
        Ok(())
    }

    fn write_sgr(&mut self, fgcol: Color, bgcol: Color) -> Result<(), TtyError> {
        try!(write!(self.outbuffer, "\x1b[3{};4{}m", (fgcol as usize) - 1, (bgcol as usize) - 1));
        Ok(())
    }

    /// Updates the size of the Terminal object to reflect that of the underlying terminal.
    /// Does not resize the buffers or clear them, just sets the size.
    fn update_size(&mut self) -> Result<(), TtyError> {
        let mut ws = WindowSize::new();
        let status = unsafe {
            ioctl::read_into::<WindowSize>(self.rawtty, TIOCGWINSZ, &mut ws)
        };
        match status {
            Ok(..) => {
                self.cols = ws.ws_col as usize;
                self.rows = ws.ws_row as usize;
            },
            Err(e) => { return Err(TtyError::from_nix(e)) },
        }
        Ok(())
    }

    fn send_char(&mut self, x: usize, y: usize, ch: char) -> Result<(), TtyError> {
        try!(self.write_cursor(Cursor::Valid(x, y)));
        self.cursor_last = Cursor::Valid(x, y);
        try!(write!(self.outbuffer, "{}", ch));
        Ok(())
    }

    fn resize(&mut self, blank: Cell) -> Result<(), TtyError> {
        self.backbuffer.resize(self.cols, self.rows, blank);
        self.frontbuffer.resize(self.cols, self.rows, blank);
        try!(self.send_clear(Cell::blank_default()));
        Ok(())
    }

    fn flush(&mut self) -> Result<(), TtyError> {
        try!(self.tty.write_all(&self.outbuffer));
        self.outbuffer.clear();
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.outbuffer.write_all(&self.device[DevFunc::ShowCursor]).unwrap();
        self.outbuffer.write_all(&self.device[DevFunc::Sgr0]).unwrap();
        self.outbuffer.write_all(&self.device[DevFunc::ClearScreen]).unwrap();
        self.outbuffer.write_all(&self.device[DevFunc::ExitCa]).unwrap();
        self.outbuffer.write_all(&self.device[DevFunc::ExitKeypad]).unwrap();
        self.outbuffer.write_all(&self.device[DevFunc::ExitMouse]).unwrap();
        self.flush().unwrap();
        termios::tcsetattr(self.rawtty, SetArg::TCSAFLUSH, &self.orig_tios).unwrap();
        SIGWINCH_STATUS.store(false, Ordering::SeqCst);
        RUSTTY_STATUS.store(false, Ordering::SeqCst);
    }
}

extern fn sigwinch_handler(_: i32) {
    SIGWINCH_STATUS.store(true, Ordering::SeqCst);
}

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

