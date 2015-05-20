use std::iter;

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
            cells: vec![vec![Cell::blank_default(); rows]; cols],
        }
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

    pub fn blank_default() -> Cell {
        Cell {
            ch: b' ',
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
    Reverse = 0x0400,
}

