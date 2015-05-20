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
    fgcolor: Color,
    bgcolor: Color,
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
            fgcolor: Color::Default,
            bgcolor: Color::Default,
        };

        try!(terminal.update_size());
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
        self.backbuf.resize(self.width, self.height);
        self.frontbuf.resize(self.width, self.height);
        Ok(())
    }

    pub fn clear(&mut self) {

    }

}

impl Drop for Terminal {
    fn drop(&mut self) {
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

