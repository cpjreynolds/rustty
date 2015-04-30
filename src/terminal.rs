use libc;
use libc::funcs::bsd44;
use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::io::*;

#[cfg(target_os="macos")]
const TIOCGWINSZ: libc::c_ulong = 0x40087468;

#[cfg(target_os="linux")]
const TIOCGWINSZ: libc::c_int = 0x5413;

pub struct Terminal {
    tty: File,
    size: WindowSize,
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
            size: WindowSize::new(),
        }
    }

    pub fn update_size(&mut self) {
        unsafe {
            bsd44::ioctl(self.tty.as_raw_fd(), TIOCGWINSZ, &self.size)
        };
    }

    pub fn rows(&self) -> u16 {
        self.size.ws_row as u16
    }

    pub fn cols(&self) -> u16 {
        self.size.ws_col as u16
    }
}

#[repr(C)]
struct WindowSize {
    ws_row: libc::c_ushort,
    ws_col: libc::c_ushort,
    ws_xpixel: libc::c_ushort,
    ws_ypixel: libc::c_ushort,
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

