use nix::sys::ioctl::*;
use nix::sys::termios;
use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::io::*;

pub use nix::Result;

pub struct Terminal {
    tty: File,
    ttyfd: RawFd,
    size: Winsize,
    orig_attr: termios::Termios,
}

impl Terminal {
    pub fn new() -> Result<Terminal> {
        let mut tmptty = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/tty").unwrap();
        let tmpttyfd = tmptty.as_raw_fd();
        let mut tmpterm = Terminal {
            tty: tmptty,
            ttyfd: tmpttyfd,
            size: Winsize::new(),
            orig_attr: termios::tcgetattr(tmpttyfd).unwrap(),
        };
        match tmpterm.update_size() {
            Ok(()) => Ok(tmpterm),
            Err(error) => Err(error),
        }
    }

    pub fn destroy(self) -> Result<()> {
        match termios::tcsetattr(self.ttyfd, termios::SetArg::TCSANOW, &self.orig_attr) {
            Ok(()) => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub fn update_size(&mut self) -> Result<()> {
        ioctl(self.tty.as_raw_fd(), TIOCGWINSZ(&mut self.size))
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

impl Constructor<Winsize> for Winsize {
    fn new() -> Winsize {
        Winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        }
    }
}

