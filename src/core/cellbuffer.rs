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
        CellBuffer::with_cell(cols, rows, Cell::default())
    }

    pub fn with_char(cols: usize, rows: usize, ch: char) -> CellBuffer {
        CellBuffer::with_cell(cols, rows, Cell::with_char(ch))
    }

    pub fn with_styles(cols: usize, rows: usize, fg: Style, bg: Style) -> CellBuffer {
        CellBuffer::with_cell(cols, rows, Cell::with_styles(fg, bg))
    }

    pub fn with_cell(cols: usize, rows: usize, cell: Cell) -> CellBuffer {
        CellBuffer {
            cols: cols,
            rows: rows,
            cells: vec![vec![cell; rows]; cols],
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

    pub fn clear(&mut self) {
        self.clear_with_cell(Cell::default());
    }

    pub fn clear_with_char(&mut self, ch: char) {
        self.clear_with_cell(Cell::with_char(ch));
    }

    pub fn clear_with_styles(&mut self, fg: Style, bg: Style) {
        self.clear_with_cell(Cell::with_styles(fg, bg));
    }

    pub fn clear_with_cell(&mut self, blank: Cell) {
        for col in &mut self.cells {
            for cell in col.iter_mut() {
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

        if newcols > self.cols {
            if newrows > self.rows {
                for col in &mut self.cells {
                    col.extend(iter::repeat(blank).take(newrows - self.rows));
                }
            } else {
                for row in &mut self.cells {
                    row.truncate(newrows);
                }
            }
            self.cells.extend(iter::repeat(vec![blank; newrows]).take(newcols - self.cols));
        } else {
            if newrows > self.rows {
                for col in &mut self.cells {
                    col.extend(iter::repeat(blank).take(newrows - self.rows));
                }
            } else {
                for col in &mut self.cells {
                    col.truncate(newrows);
                }
            }
            self.cells.truncate(newcols);
        }

        self.rows = newrows;
        self.cols = newcols;
    }
}

impl Default for CellBuffer {
    fn default() -> CellBuffer {
        CellBuffer::with_cell(0, 0, Cell::default())
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
    ch: char,
    fg: Style,
    bg: Style,
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
        Cell::new(ch, Style::default(), Style::default())
    }

    pub fn with_styles(fg: Style, bg: Style) -> Cell {
        Cell::new(' ', fg, bg)
    }

    pub fn ch(&self) -> char {
        self.ch
    }

    pub fn set_ch(&mut self, newch: char) {
        self.ch = newch;
    }

    pub fn fg(&self) -> Style {
        self.fg
    }

    pub fn set_fg(&mut self, newfg: Style) {
        self.fg = newfg;
    }

    pub fn bg(&self) -> Style {
        self.bg
    }

    pub fn set_bg(&mut self, newbg: Style) {
        self.bg = newbg;
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::new(' ', Style::default(), Style::default())
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Style(Color, Attr);

impl Style {
    pub fn new(color: Color, attr: Attr) -> Style {
        Style(color, attr)
    }

    pub fn with_color(c: Color) -> Style {
        Style::new(c, Attr::Default)
    }

    pub fn with_attr(a: Attr) -> Style {
        Style::new(Color::Default, a)
    }

    pub fn color(&self) -> Color {
        self.0
    }

    pub fn set_color(&mut self, newcolor: Color) {
        self.0 = newcolor;
    }

    pub fn attr(&self) -> Attr {
        self.1
    }

    pub fn set_attr(&mut self, newattr: Attr) {
        self.1 = newattr;
    }
}

impl Default for Style {
    fn default() -> Style {
        Style::new(Color::Default, Attr::Default)
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

