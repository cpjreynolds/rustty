#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate nix;

mod core;
mod util;

pub use core::Terminal;
pub use util::Error;
