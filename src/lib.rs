#![doc(html_root_url = "http://cpjreynolds.github.io/rustty/rustty/index.html")]

//! Terminal UI library.

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate nix;

mod core;
mod util;

pub use core::terminal::Terminal;
pub use core::cellbuffer::{
    Cell,
    Style,
    Color,
    Attr,
};
pub use core::cursor::Cursor;
pub use util::error::Error;
