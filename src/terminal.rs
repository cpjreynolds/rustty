use nix;
use nix::sys::ioctl;
use nix::sys::termios;
use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::io::*;
use std::result;

const TTYPATH: &'static str = "/dev/tty";

pub struct Terminal {
    tty: File,
    ttyfd: RawFd,
    size: ioctl::Winsize,
    orig_attr: termios::Termios,
}

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    Unknown,
}

impl Terminal {
    pub fn new() -> nix::Result<Terminal> {
        let mut tmptty = OpenOptions::new()
            .read(true)
            .write(true)
            .open(TTYPATH).unwrap();
        let tmpttyfd = tmptty.as_raw_fd();
        let mut tmpterm = Terminal {
            tty: tmptty,
            ttyfd: tmpttyfd,
            size: ioctl::Winsize::new(),
            orig_attr: termios::tcgetattr(tmpttyfd).unwrap(),
        };
        match tmpterm.update_size() {
            Ok(()) => Ok(tmpterm),
            Err(error) => Err(error),
        }
    }

    pub fn destroy(self) -> nix::Result<()> {
        match termios::tcsetattr(self.ttyfd, termios::SetArg::TCSANOW, &self.orig_attr) {
            Ok(()) => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub fn update_size(&mut self) -> nix::Result<()> {
        ioctl::ioctl(self.tty.as_raw_fd(), ioctl::TIOCGWINSZ(&mut self.size))
    }

    pub fn rows(&self) -> u16 {
        self.size.ws_row as u16
    }

    pub fn cols(&self) -> u16 {
        self.size.ws_col as u16
    }
}

trait Constructor<T> {
    fn new() -> T;
}

impl Constructor<ioctl::Winsize> for ioctl::Winsize {
    fn new() -> ioctl::Winsize {
        ioctl::Winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        }
    }
}

