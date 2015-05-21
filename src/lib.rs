//! Terminal UI library.

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate nix;

pub mod core;
pub mod util;

pub use core::{Terminal, Cell, Color, Style, Attr};
pub use util::Error;
