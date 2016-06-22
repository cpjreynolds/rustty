use std::io::Error;
use std::os::unix::io::RawFd;
use std::mem;

use libc;

/// Controller for low-level interaction with a terminal device.
pub struct TermCtl {
    fd: RawFd,
    orig_tios: libc::termios,
}

impl TermCtl {
    pub fn new(fd: RawFd) -> Result<TermCtl, Error> {
        let mut termios = unsafe { mem::uninitialized() };

        let res = unsafe { libc::tcgetattr(fd, &mut termios) };

        if res != 0 {
            Err(Error::last_os_error())
        } else {
            Ok(TermCtl {
                fd: fd,
                orig_tios: termios,
            })
        }
    }

    pub fn set(&self) -> Result<(), Error> {
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

        let res = unsafe { libc::tcsetattr(self.fd, libc::TCSAFLUSH, &tios) };

        if res != 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub fn window_size(&self) -> Result<(usize, usize), Error> {
        let mut ws: libc::winsize = unsafe { mem::uninitialized() };
        let res = unsafe { libc::ioctl(self.fd, libc::TIOCGWINSZ, &mut ws) };
        if res != 0 {
            Err(Error::last_os_error())
        } else {
            Ok((ws.ws_col as usize, ws.ws_row as usize))
        }
    }

    pub fn reset(&self) -> Result<(), Error> {
        let res = unsafe { libc::tcsetattr(self.fd, libc::TCSAFLUSH, &self.orig_tios) };
        if res != 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}
