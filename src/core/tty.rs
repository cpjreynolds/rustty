use std::os::unix::io::{AsRawFd, RawFd};
use std::fs::{File, OpenOptions};
use std::mem;
use std::io::prelude::*;
use std::io::{Error, Result};

use libc;

// Low-level interface with the underlying terminal.
pub struct RawTerminal {
    io: File,
    // Original `termios` struct.
    orig_termios: Termios,
    // Current `termios` struct.
    termios: Termios,
}

impl RawTerminal {
    pub fn new() -> Result<RawTerminal> {
        let io = try!(OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/tty"));

        let orig_termios = try!(Termios::from_fd(io.as_raw_fd()));

        Ok(RawTerminal {
            io: io,
            orig_termios: orig_termios.clone(),
            termios: orig_termios,
        })
    }

    pub fn termios(&self) -> Termios {
        self.termios.clone()
    }

    pub fn set_termios(&mut self, tios: Termios) -> Result<()> {
        tios.apply(self.io.as_raw_fd())
    }

    pub fn window_size(&self) -> Result<(usize, usize)> {
        let fd = self.io.as_raw_fd();
        let mut ws: libc::winsize = unsafe { mem::uninitialized() };

        let ret = unsafe { libc::ioctl(fd, libc::TIOCGWINSZ, &mut ws) };
        if ret != 0 {
            Err(Error::last_os_error())
        } else {
            Ok((ws.ws_col as usize, ws.ws_row as usize))
        }
    }
}

impl Write for RawTerminal {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.io.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.io.flush()
    }
}

impl Read for RawTerminal {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.io.read(buf)
    }
}

impl AsRawFd for RawTerminal {
    fn as_raw_fd(&self) -> RawFd {
        self.io.as_raw_fd()
    }
}

impl Drop for RawTerminal {
    fn drop(&mut self) {
        let fd = self.io.as_raw_fd();
        let _ = self.orig_termios.apply(fd);
    }
}

#[derive(Clone)]
pub struct Termios(libc::termios);

impl Termios {
    // Retrieve the `termios` of the specified file descriptor.
    pub fn from_fd(fd: RawFd) -> Result<Termios> {
        let mut tios = unsafe { mem::uninitialized() };
        let ret = unsafe { libc::tcgetattr(fd, &mut tios) };
        if ret != 0 {
            Err(Error::last_os_error())
        } else {
            Ok(Termios(tios))
        }
    }

    // Apply the `termios` to the specified file descriptor.
    pub fn apply(&self, fd: RawFd) -> Result<()> {
        let ret = unsafe { libc::tcsetattr(fd, libc::TCSAFLUSH, &self.0) };

        if ret != 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub fn iflags_mut(&mut self) -> &mut InputFlags {
        unsafe { mem::transmute(&mut self.0.c_iflag) }
    }

    pub fn oflags_mut(&mut self) -> &mut OutputFlags {
        unsafe { mem::transmute(&mut self.0.c_oflag) }
    }
    pub fn lflags_mut(&mut self) -> &mut LocalFlags {
        unsafe { mem::transmute(&mut self.0.c_lflag) }
    }

    pub fn cflags_mut(&mut self) -> &mut ControlFlags {
        unsafe { mem::transmute(&mut self.0.c_cflag) }
    }

    pub fn set_cc(&mut self, cc_idx: ControlChar, val: libc::cc_t) {
        self.0.c_cc[cc_idx as usize] = val;
    }
}

bitflags! {
    pub flags InputFlags: libc::tcflag_t {
        const IGNBRK = libc::IGNBRK,
        const BRKINT = libc::BRKINT,
        const PARMRK = libc::PARMRK,
        const ISTRIP = libc::ISTRIP,
        const INLCR = libc::INLCR,
        const IGNCR = libc::IGNCR,
        const ICRNL = libc::ICRNL,
        const IXON = libc::IXON,
    }
}

bitflags! {
    pub flags OutputFlags: libc::tcflag_t {
        const OPOST = libc::OPOST,
    }
}

bitflags! {
    pub flags LocalFlags: libc::tcflag_t {
        const ECHO = libc::ECHO,
        const ECHONL = libc::ECHONL,
        const ICANON = libc::ICANON,
        const ISIG = libc::ISIG,
        const IEXTEN = libc::IEXTEN,
    }
}

bitflags! {
    pub flags ControlFlags: libc::tcflag_t {
        const CSIZE = libc::CSIZE,
        const PARENB = libc::PARENB,
        const CS8 = libc::CS8,
    }
}

#[repr(usize)]
pub enum ControlChar {
    VMIN = libc::VMIN,
    VTIME = libc::VTIME,
}
