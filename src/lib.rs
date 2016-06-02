#![doc(html_root_url = "http://cpjreynolds.github.io/rustty/rustty/index.html")]

//! # Rustty
//!
//! Rustty is a terminal UI library that provides a simple, concise abstraction over an
//! underlying terminal device.
//!
//! Rustty is based on the concepts of cells and events. A terminal display is an array of cells,
//! each holding a character and a set of foreground and background styles. Events are how a
//! terminal communicates changes in its state; events are received from a terminal, processed, and
//! pushed onto an input stream to be read and responded to.
//!
//! Futher reading on the concepts behind Rustty can be found in the
//! [README](https://github.com/cpjreynolds/rustty/blob/master/README.md)

extern crate term;
extern crate libc;
extern crate gag;

mod core;
pub mod ui;

pub use core::terminal::Terminal;
pub use core::cellbuffer::{Cell, Color, Attr, CellAccessor};
pub use core::position::{Pos, Size, HasSize, HasPosition};
pub use core::input::Event;
