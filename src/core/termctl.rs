use std::io::Error;
use std::os::unix::io::RawFd;
use std::mem;

use nix::sys::termios;
use nix::sys::termios::{IGNBRK, BRKINT, PARMRK, ISTRIP, INLCR, IGNCR, ICRNL, IXON};
use nix::sys::termios::{OPOST, ECHO, ECHONL, ICANON, ISIG, IEXTEN, CSIZE, PARENB, CS8};
use nix::sys::termios::{VMIN, VTIME};
use nix::sys::termios::SetArg;
use nix::sys::termios::Termios;

mod ffi {
    use libc;

    #[cfg(target_os = "macos")]
    pub const TIOCGWINSZ: libc::c_ulong = 0x40087468;
    #[cfg(target_os = "linux")]
    pub const TIOCGWINSZ: libc::c_ulong = 0x00005413;

    #[repr(C)]
    #[derive(Debug, Clone)]
    pub struct winsize {
        pub ws_row: u16,
        pub ws_col: u16,
        ws_xpixel: u16,
        ws_ypixel: u16,
    }

    extern "C" {
        pub fn ioctl(fd: libc::c_int, req: libc::c_ulong, ...) -> libc::c_int;
    }
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
        tios.c_iflag = tios.c_iflag &
                       !(IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON);
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
        let mut ws: ffi::winsize = unsafe { mem::uninitialized() };
        try!(unsafe { convert_ioctl_res!((ffi::ioctl(self.fd, ffi::TIOCGWINSZ, &mut ws))) });
        Ok((ws.ws_col as usize, ws.ws_row as usize))
    }

    pub fn reset(&self) -> Result<(), Error> {
        try!(termios::tcsetattr(self.fd, SetArg::TCSAFLUSH, &self.orig_tios));
        Ok(())
    }
}
