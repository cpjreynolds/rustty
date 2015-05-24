#![feature(io)]
#![doc(html_root_url = "http://cpjreynolds.github.io/rustty/rustty/index.html")]

//! Terminal UI library.

extern crate nix;
extern crate time;

mod core;
mod util;

pub use core::terminal::Terminal;
pub use core::cellbuffer::{
    Cell,
    Style,
    Color,
    Attr,
};
pub use core::input::Event;
pub use core::cursor::Cursor;
pub use util::error::Error;

