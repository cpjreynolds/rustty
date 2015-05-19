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

static SIGWINCH_STATUS: AtomicBool = ATOMIC_BOOL_INIT;
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
    width: u16,
    height: u16,
    device: &'static Device,
}

impl Terminal {
    pub fn new() -> Result<Terminal, TtyError> {
        if RUSTTY_STATUS.compare_and_swap(false, true, Ordering::SeqCst) {
            return Err(TtyError::new("Rustty already initialized"))
        }
        let tty = OpenOptions::new()
            .write(true)
            .read(true)
            .open("/dev/tty")
            .unwrap();
        let device = Device::new().unwrap();
        let sa_winch = signal::SigAction::new(sigwinch_handler, SockFlag::empty(), SigSet::empty());
        unsafe {
            if let Err(e) = signal::sigaction(SIGWINCH, &sa_winch) {
                panic!("{:?}", e);
            }
        }
        let orig_tios = termios::tcgetattr(tty.as_raw_fd()).unwrap();
        let mut tios = orig_tios.clone();
        tios.c_iflag = tios.c_iflag & !(IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON);
        tios.c_oflag = tios.c_oflag & !OPOST;
        tios.c_lflag = tios.c_lflag & !(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
        tios.c_cflag = tios.c_cflag & !(CSIZE | PARENB);
        tios.c_cflag = tios.c_cflag | CS8;
        tios.c_cc[VMIN] = 0;
        tios.c_cc[VTIME] = 0;
        termios::tcsetattr(tty.as_raw_fd(), SetArg::TCSAFLUSH, &tios).unwrap();
        let ttyfd = tty.as_raw_fd();
        let mut terminal = Terminal {
            orig_tios: orig_tios,
            tios: tios,
            tty: tty,
            ttyfd: ttyfd,
            width: 0,
            height: 0,
            device: device,
        };
        terminal.update_size().unwrap();
        Ok(terminal)
    }

    pub fn clear(&mut self) {
        write!(self.tty, "{}", &self.device[DFunction::ClearScreen]).unwrap();
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    fn update_size(&mut self) -> Result<(), TtyError> {
        let mut ws = WindowSize::new();
        let status = unsafe {
            ioctl::read_into::<WindowSize>(self.ttyfd, TIOCGWINSZ, &mut ws)
        };
        match status {
            Ok(..) => {
                self.width = ws.ws_row;
                self.height = ws.ws_col;
                Ok(())
            },
            Err(e) => { Err(TtyError::from_nix(e)) },
        }
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

