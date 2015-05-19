pub struct CellBuffer<'a> {
    width: u32,
    height: u32,
    cells: &'a [Cell],
}

pub struct Cell {
    ch: char,
    fg: Style,
    bg: Style,
}

pub struct Style {
    color: Color,
    attribute: Attribute,
}

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

pub enum Attribute {
    Default = 0b000,
    Bold = 0b001,
    Underline = 0b010,
    Reverse = 0b100,
}
