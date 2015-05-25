#![feature(io)]
#![feature(collections)]
#![doc(html_root_url = "http://cpjreynolds.github.io/rustty/rustty/index.html")]

//! # Rustty
//!
//! Rustty is a terminal UI library that provides a simple and concise abstraction over an
//! underlying terminal device.
//!
//! Rustty is based on the concepts of cells and events. The terminal display is an array of cells,
//! each holding a character and a set of foreground and background styles. Events are how a
//! terminal communicates changes in its state; each event represents some form of action taken by
//! the user, such as a keypress. Each terminal has an event stream that receives input events and
//! buffers them until they are read.
//!
//! More on the concepts of Rustty can be found in the
//! [README](https://github.com/cpjreynolds/rustty/blob/master/README.md)

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
pub use core::input::Event;
pub use util::error::Error;

