use std::os::unix::io::RawFd;
use std::mem;

use nix::sys::termios;
use nix::sys::termios::{IGNBRK, BRKINT, PARMRK, ISTRIP, INLCR, IGNCR, ICRNL, IXON};
use nix::sys::termios::{OPOST, ECHO, ECHONL, ICANON, ISIG, IEXTEN, CSIZE, PARENB, CS8};
use nix::sys::termios::{VMIN, VTIME};
use nix::sys::termios::SetArg;
use nix::sys::termios::Termios;
use nix::sys::ioctl;

use util::errors::Error;

#[cfg(target_os = "macos")]
const TIOCGWINSZ: u64 = 0x40087468;
#[cfg(target_os = "linux")]
const TIOCGWINSZ: u64 = 0x00005413;

#[repr(C)]
#[derive(Debug, Clone)]
struct WindowSize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}

/// Controller for low-level interaction with a terminal device.
pub struct TermCtl {
    fd: RawFd,
    orig_tios: Termios,
}

impl TermCtl {
    pub fn new(fd: RawFd) -> Result<TermCtl, Error> {
        Ok(TermCtl {
            fd: fd,
            orig_tios: try!(termios::tcgetattr(fd)),
        })
    }

    pub fn set(&self) -> Result<(), Error> {
        let mut tios = self.orig_tios.clone();
        tios.c_iflag = tios.c_iflag & !(IGNBRK | BRKINT | PARMRK | ISTRIP |
                                        INLCR | IGNCR | ICRNL | IXON);
        tios.c_oflag = tios.c_oflag & !OPOST;
        tios.c_lflag = tios.c_lflag & !(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
        tios.c_cflag = tios.c_cflag & !(CSIZE | PARENB);
        tios.c_cflag = tios.c_cflag | CS8;
        tios.c_cc[VMIN] = 0;
        tios.c_cc[VTIME] = 0;

        try!(termios::tcsetattr(self.fd, SetArg::TCSAFLUSH, &tios));
        Ok(())
    }

    pub fn window_size(&self) -> Result<(usize, usize), Error> {
        let mut ws = unsafe { mem::uninitialized() };
        try!(unsafe {
            ioctl::read_into::<WindowSize>(self.fd, TIOCGWINSZ, &mut ws)
        });
        Ok((ws.ws_col as usize, ws.ws_row as usize))
    }

    pub fn reset(&self) -> Result<(), Error> {
        try!(termios::tcsetattr(self.fd, SetArg::TCSAFLUSH, &self.orig_tios));
        Ok(())
    }
}
