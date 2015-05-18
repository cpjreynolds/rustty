use std::io::prelude::*;
use std::fs::OpenOptions;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

use nix::sys::termios;
use nix::sys::termios::{IGNBRK, BRKINT, PARMRK, ISTRIP, INLCR, IGNCR, ICRNL, IXON};
use nix::sys::termios::{OPOST, ECHO, ECHONL, ICANON, ISIG, IEXTEN, CSIZE, PARENB, CS8};
use nix::sys::termios::{VMIN, VTIME};
use nix::sys::termios::SetArg;
use nix::sys::signal;
use nix::sys::signal::{SockFlag, SigSet};
use nix::sys::signal::signal::SIGWINCH;

use Device;
use TtyError;

static SIGWINCH_STATUS: AtomicBool = ATOMIC_BOOL_INIT;
static RUSTTY_STATUS: AtomicBool = ATOMIC_BOOL_INIT;

pub struct Terminal {
    orig_tios: termios::Termios,
    tios: termios::Termios,
    tty: File,
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

        let device = Device::get().unwrap();

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

        Ok(Terminal {
            orig_tios: orig_tios,
            tios: tios,
            tty: tty,
        })
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        termios::tcsetattr(self.tty.as_raw_fd(), SetArg::TCSAFLUSH, &self.orig_tios).unwrap();
        RUSTTY_STATUS.store(false, Ordering::SeqCst);
    }
}

extern fn sigwinch_handler(_: i32) {
    SIGWINCH_STATUS.store(true, Ordering::SeqCst);
}

