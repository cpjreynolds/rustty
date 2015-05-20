use std::iter::repeat;

#[derive(Clone, PartialEq, Eq)]
pub struct CellBuffer {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl CellBuffer {
    pub fn new(width: usize, height: usize) -> CellBuffer {
        CellBuffer {
            width: width,
            height: height,
            cells: Vec::with_capacity(width * height),
        }
    }

    pub fn clear(&mut self, new: Cell) {
        for cell in self.cells.iter_mut() {
            cell.ch = new.ch;
            cell.fg = new.fg;
            cell.bg = new.bg;
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if (self.width == width && self.height == height) {
            return;
        }

        let oldw = self.width;
        let oldh = self.height;

        let mut newbuf: Vec<Cell> = Vec::with_capacity(width * height);

        let minw = if width < oldw { width } else { oldw };
        let minh = if height < oldh { height } else { oldh };

        for i in 0..minh {
            newbuf[i * width] = self.cells[i * oldw];
        }
        self.cells = newbuf;
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Cell {
    ch: u8,
    fg: Style,
    bg: Style,
}

impl Cell {
    pub fn blank(fg: Style, bg: Style) -> Cell {
        Cell {
            ch: b' ',
            fg: fg,
            bg: bg,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Style(Color, Attr);

impl Style {
    pub fn default() -> Style {
        Style(Color::Default, Attr::Default)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Attr {
    Default = 0x0000,
    Bold = 0x0100,
    Underline = 0x0200,
    BoldUnderline = 0x300,
    Reverse = 0x0400,
    BoldReverse = 0x0500,
    UnderlineReverse = 0x0600,
    BoldUnderlineReverse = 0x0700,
}

