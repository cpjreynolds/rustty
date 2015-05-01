use nix::sys::ioctl::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::io::*;

pub use nix::Result;

pub struct Terminal {
    tty: File,
    size: Winsize,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            tty: {
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open("/dev/tty").unwrap()
            },
            size: Winsize::new(),
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
