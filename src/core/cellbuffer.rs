use std::iter;
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq)]
pub struct CellBuffer {
    cols: usize,
    rows: usize,
    cells: Vec<Vec<Cell>>,
}

impl CellBuffer {
    pub fn new(cols: usize, rows: usize) -> CellBuffer {
        CellBuffer {
            cols: cols,
            rows: rows,
            cells: vec![vec![Cell::default(); cols]; rows],
        }
    }

    pub fn with_char(cols: usize, rows: usize, ch: char) -> CellBuffer {
        CellBuffer {
            cols: cols,
            rows: rows,
            cells: vec![vec![Cell::with_char(ch); cols]; rows],
        }
    }

    pub fn with_styles(cols: usize, rows: usize, fg: Style, bg: Style) -> CellBuffer {
        CellBuffer {
            cols: cols,
            rows: rows,
            cells: vec![vec![Cell::with_styles(fg, bg); cols]; rows],
        }
    }

    pub fn with_cell(cols: usize, rows: usize, cell: Cell) -> CellBuffer {
        CellBuffer {
            cols: cols,
            rows: rows,
            cells: vec![vec![cell; cols]; rows],
        }
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn size(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }

    pub fn clear(&mut self, blank: Cell) {
        for row in &mut self.cells {
            for cell in row.iter_mut() {
                cell.ch = blank.ch;
                cell.fg = blank.fg;
                cell.bg = blank.bg;
            }
        }
    }

    pub fn resize(&mut self, newcols: usize, newrows: usize, blank: Cell) {
        if self.cols == newcols && self.rows == newrows {
            return;
        }

        if newrows > self.rows {
            if newcols > self.cols {
                for row in &mut self.cells {
                    row.extend(iter::repeat(blank).take(newcols - self.cols));
                }
            } else {
                for row in &mut self.cells {
                    row.truncate(newcols);
                }
            }
            self.cells.extend(iter::repeat(vec![blank; newcols]).take(newrows - self.rows));
        } else {
            if newcols > self.cols {
                for row in &mut self.cells {
                    row.extend(iter::repeat(blank).take(newcols - self.cols));
                }
            } else {
                for row in &mut self.cells {
                    row.truncate(newcols);
                }
            }
            self.cells.truncate(newrows);
        }

        self.rows = newrows;
        self.cols = newcols;
    }
}

impl Index<usize> for CellBuffer {
    type Output = Vec<Cell>;

    fn index(&self, index: usize) -> &Vec<Cell> {
        &self.cells[index]
    }
}

impl IndexMut<usize> for CellBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Vec<Cell> {
        &mut self.cells[index]
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: Style,
    pub bg: Style,
}

impl Cell {
    pub fn new(ch: char, fg: Style, bg: Style) -> Cell {
        Cell {
            ch: ch,
            fg: fg,
            bg: bg,
        }
    }

    pub fn with_char(ch: char) -> Cell {
        Cell {
            ch: ch,
            fg: Style::default(),
            bg: Style::default(),
        }
    }

    pub fn with_styles(fg: Style, bg: Style) -> Cell {
        Cell {
            ch: ' ',
            fg: fg,
            bg: bg,
        }
    }

    pub fn default() -> Cell {
        Cell {
            ch: ' ',
            fg: Style::default(),
            bg: Style::default(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Style(pub Color, pub Attr);

impl Style {
    pub fn default() -> Style {
        Style(Color::Default, Attr::Default)
    }

    pub fn with_color(c: Color) -> Style {
        Style(c, Attr::Default)
    }

    pub fn with_attr(a: Attr) -> Style {
        Style(Color::Default, a)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Black = 0x0000,
    Red = 0x0001,
    Green = 0x0002,
    Yellow = 0x0003,
    Blue = 0x0004,
    Magenta = 0x0005,
    Cyan = 0x0006,
    White = 0x0007,
    Default,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Attr {
    Default,
    Bold,
    Underline,
    Reverse,
}

