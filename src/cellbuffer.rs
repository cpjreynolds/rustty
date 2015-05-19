use std::iter::repeat;

#[derive(Clone, PartialEq, Eq)]
pub struct CellBuffer {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl CellBuffer {
    pub fn new(x: usize, y: usize) -> CellBuffer {
        CellBuffer {
            width: x,
            height: y,
            cells: Vec::with_capacity(x * y),
        }
    }
}

// Using until resize hits stable
pub trait Resizable<T> {
    fn resize(&mut self, new_len: usize, value: T);
}

impl Resizable<Cell> for Vec<Cell> {
    fn resize(&mut self, new_len: usize, value: Cell) {
        let len = self.len();

        if new_len > len {
            self.extend(repeat(value).take(new_len - len));
        } else {
            self.truncate(new_len);
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Cell {
    ch: u8,
    fg: Style,
    bg: Style,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Style {
    color: Color,
    attribute: Attribute,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Color {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Attribute {
    Default = 0x0000,
    Bold = 0x0100,
    Underline = 0x0200,
    BoldUnderline = 0x300,
    Reverse = 0x0400,
    BoldReverse = 0x0500,
    UnderlineReverse = 0x0600,
    BoldUnderlineReverse = 0x0700,
}

