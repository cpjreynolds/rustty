extern crate rustty;

use rustty::terminal::Window;

fn main() {
    let mut term = Window::new();
    term.update_size();
    println!("{} {}", term.rows(), term.cols());
    term.update_size();
    println!("{} {}", term.rows(), term.cols());
    println!("At end");
}