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
use DFunction;
use TtyError;
use CellBuffer;
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

pub struct Terminal {
    orig_tios: termios::Termios,
    tios: termios::Termios,
    tty: File,
    ttyfd: RawFd,
    width: usize,
    height: usize,
    device: &'static Device,
    frontbuf: CellBuffer,
    backbuf: CellBuffer,
    outbuf: Vec<u8>,
    fg: Style,
    bg: Style,
}

impl Terminal {
    pub fn new() -> Result<Terminal, TtyError> {
        if RUSTTY_STATUS.compare_and_swap(false, true, Ordering::SeqCst) {
            return Err(TtyError::new("Rustty already initialized"))
        }

        let device = try!(Device::new());

        let tty = try!(OpenOptions::new()
            .write(true)
            .read(true)
            .open("/dev/tty"));

        let ttyfd = tty.as_raw_fd();

        let sa_winch = signal::SigAction::new(sigwinch_handler, SockFlag::empty(), SigSet::empty());
        unsafe {
            if let Err(e) = signal::sigaction(SIGWINCH, &sa_winch) {
                panic!("{:?}", e);
            }
        }

        let orig_tios = termios::tcgetattr(ttyfd).unwrap();

        let mut tios = orig_tios.clone();
        tios.c_iflag = tios.c_iflag & !(IGNBRK | BRKINT | PARMRK | ISTRIP |
                                        INLCR | IGNCR | ICRNL | IXON);
        tios.c_oflag = tios.c_oflag & !OPOST;
        tios.c_lflag = tios.c_lflag & !(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
        tios.c_cflag = tios.c_cflag & !(CSIZE | PARENB);
        tios.c_cflag = tios.c_cflag | CS8;
        tios.c_cc[VMIN] = 0;
        tios.c_cc[VTIME] = 0;
        termios::tcsetattr(ttyfd, SetArg::TCSAFLUSH, &tios).unwrap();

        let mut terminal = Terminal {
            orig_tios: orig_tios,
            tios: tios,
            tty: tty,
            ttyfd: ttyfd,
            width: 0,
            height: 0,
            device: device,
            frontbuf: CellBuffer::new(0, 0),
            backbuf: CellBuffer::new(0, 0),
            outbuf: Vec::with_capacity(32 * 1024),
            fg: Style::default(),
            bg: Style::default(),
        };
        try!(terminal.outbuf.write_all(&terminal.device[DFunction::EnterCa]));
        try!(terminal.outbuf.write_all(&terminal.device[DFunction::EnterKeypad]));
        try!(terminal.outbuf.write_all(&terminal.device[DFunction::HideCursor]));
        try!(terminal.send_clear(Style::default(), Style::default()));

        try!(terminal.update_size());
        terminal.clear();
        Ok(terminal)
    }

    /// Returns the width of the terminal.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the terminal.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the size of the terminal as (x, y).
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn clear(&mut self) {
        self.backbuf.clear(Cell::blank(self.fg, self.bg));
    }

    pub fn send_clear(&mut self, fg: Style, bg: Style) -> Result<(), TtyError> {
        try!(self.send_style(fg, bg));
        try!(self.outbuf.write_all(&self.device[DFunction::ClearScreen]));
        try!(self.flush());
        self.outbuf.clear();
        Ok(())
    }

    pub fn send_style(&mut self, fg: Style, bg: Style) -> Result<(), TtyError> {
        try!(self.outbuf.write_all(&self.device[DFunction::Sgr0]));
        let Style(fgcol, fgattr) = fg;
        let Style(bgcol, bgattr) = bg;

        match fgattr {
            Attr::Bold => try!(self.outbuf.write_all(&self.device[DFunction::Bold])),
            Attr::Underline => try!(self.outbuf.write_all(&self.device[DFunction::Underline])),
            Attr::Reverse => try!(self.outbuf.write_all(&self.device[DFunction::Reverse])),
            _ => {},
        }

        match bgattr {
            Attr::Bold => try!(self.outbuf.write_all(&self.device[DFunction::Blink])),
            Attr::Underline => {},
            Attr::Reverse => try!(self.outbuf.write_all(&self.device[DFunction::Reverse])),
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

    pub fn write_sgr_fg(&mut self, fgcol: Color) -> Result<(), TtyError> {
        try!(write!(self.outbuf, "\x1b[3{}m", (fgcol as usize) - 1));
        Ok(())
    }

    pub fn write_sgr_bg(&mut self, bgcol: Color) -> Result<(), TtyError> {
        try!(write!(self.outbuf, "\x1b[4{}m", (bgcol as usize) - 1));
        Ok(())
    }

    pub fn write_sgr(&mut self, fgcol: Color, bgcol: Color) -> Result<(), TtyError> {
        try!(write!(self.outbuf, "\x1b[3{};4{}m", (fgcol as usize) - 1, (bgcol as usize) - 1));
        Ok(())
    }

    /// Updates the size of the Terminal object to reflect that of the underlying terminal.
    /// Resizes the cellbuffer as well.
    pub fn update_size(&mut self) -> Result<(), TtyError> {
        let mut ws = WindowSize::new();
        let status = unsafe {
            ioctl::read_into::<WindowSize>(self.ttyfd, TIOCGWINSZ, &mut ws)
        };
        match status {
            Ok(..) => {
                self.width = ws.ws_col as usize;
                self.height = ws.ws_row as usize;
            },
            Err(e) => { return Err(TtyError::from_nix(e)) },
        }
        let blank = Cell::blank(self.fg, self.bg);
        self.backbuf.resize(self.width, self.height, blank);
        self.frontbuf.resize(self.width, self.height, blank);
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), TtyError> {
        try!(self.tty.write_all(&self.outbuf));
        self.outbuf.clear();
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.outbuf.write_all(&self.device[DFunction::ShowCursor]).unwrap();
        self.outbuf.write_all(&self.device[DFunction::Sgr0]).unwrap();
        self.outbuf.write_all(&self.device[DFunction::ClearScreen]).unwrap();
        self.outbuf.write_all(&self.device[DFunction::ExitCa]).unwrap();
        self.outbuf.write_all(&self.device[DFunction::ExitKeypad]).unwrap();
        self.outbuf.write_all(&self.device[DFunction::ExitMouse]).unwrap();
        self.flush().unwrap();
        termios::tcsetattr(self.ttyfd, SetArg::TCSAFLUSH, &self.orig_tios).unwrap();
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

