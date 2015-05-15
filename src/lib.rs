#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate nix;

use std::io::prelude::*;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;

use nix::sys::termios;

mod device;

pub fn init() -> Terminal {
    let tty = OpenOptions::new()
                          .read(true)
                          .write(true)
                          .open("/dev/tty")
                          .unwrap();

    Terminal {
        orig_tios: termios::tcgetattr(tty.as_raw_fd()).unwrap(),
    }
}

pub struct Terminal {
    orig_tios: termios::Termios,
}

