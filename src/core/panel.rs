use std::ops::{Index, IndexMut, Deref, DerefMut};
use std::sync::Arc;
use std::rc::Rc;
use std::borrow::Cow;
use std::cmp;

use core::cell::Cell;


pub trait Draw {
    // Draw `Self` at the coordinates `coords`, on the `target` `Panel`.
    fn draw(&self, x: usize, y: usize, target: &mut Panel);
}

impl<'a, T: ?Sized> Draw for &'a T
    where T: Draw
{
    fn draw(&self, x: usize, y: usize, target: &mut Panel) {
        Draw::draw(&**self, x, y, target);
    }
}

impl<'a, T: ?Sized> Draw for &'a mut T
    where T: Draw
{
    fn draw(&self, x: usize, y: usize, target: &mut Panel) {
        Draw::draw(&**self, x, y, target);
    }
}

impl<'a, B: ?Sized> Draw for Cow<'a, B>
    where B: Draw + ToOwned,
          B::Owned: Draw
{
    fn draw(&self, x: usize, y: usize, target: &mut Panel) {
        match *self {
            Cow::Borrowed(ref b) => Draw::draw(b, x, y, target),
            Cow::Owned(ref o) => Draw::draw(o, x, y, target),
        }
    }
}

impl<T: ?Sized> Draw for Arc<T>
    where T: Draw
{
    fn draw(&self, x: usize, y: usize, target: &mut Panel) {
        Draw::draw(&**self, x, y, target);
    }
}

impl<T: ?Sized> Draw for Rc<T>
    where T: Draw
{
    fn draw(&self, x: usize, y: usize, target: &mut Panel) {
        Draw::draw(&**self, x, y, target);
    }
}

impl<T: ?Sized> Draw for Box<T>
    where T: Draw
{
    fn draw(&self, x: usize, y: usize, target: &mut Panel) {
        Draw::draw(&**self, x, y, target);
    }
}

impl Draw for str {
    fn draw(&self, x: usize, y: usize, target: &mut Panel) {
        let offset = target.offset(x, y);

        // Iterator over the target cells.
        let cells = target.iter_mut().skip(offset).take(self.len());

        for (cell, ch) in cells.zip(self.chars()) {
            cell.set_ch(ch);
        }
    }
}

impl Draw for char {
    fn draw(&self, x: usize, y: usize, target: &mut Panel) {
        target.get_mut(x, y).map(|cell| cell.set_ch(*self));
    }
}

impl Draw for Panel {
    fn draw(&self, x: usize, y: usize, target: &mut Panel) {
        let tcols = target.cols();
        let scols = self.cols();
        let srows = self.rows();
        // First get the y-axis.
        let tlines = target.chunks_mut(tcols).skip(y).take(srows);
        // Now the x-axis. `tlines` is now an iterator of lines, which in turn are iterators of
        // cells.
        let tlines = tlines.map(|line| line.iter_mut().skip(x).take(scols));
        // Source lines.
        let slines = self.chunks(scols);

        for (tline, sline) in tlines.zip(slines) {
            for (tcell, scell) in tline.zip(sline) {
                *tcell = *scell;
            }
        }

    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Panel {
    buf: Vec<Cell>,
    cols: usize,
    rows: usize,
}

impl Panel {
    pub fn new() -> Panel {
        Panel {
            buf: Vec::new(),
            cols: 0,
            rows: 0,
        }
    }

    pub fn with_size(cols: usize, rows: usize, value: Cell) -> Panel {
        let len = cols * rows;
        let mut buf = Vec::with_capacity(len);
        buf.resize(len, value);

        Panel {
            cols: cols,
            rows: rows,
            buf: buf,
        }
    }

    pub fn offset(&self, x: usize, y: usize) -> usize {
        (self.cols * y) + x
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns the size of the `Panel` as `(cols, rows)`.
    pub fn size(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }

    pub fn clear(&mut self, value: Cell) {
        for cell in &mut self.buf {
            *cell = value;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.cols && y < self.rows {
            let offset = self.offset(x, y);
            self.buf.get(offset)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x < self.cols && y < self.rows {
            let offset = self.offset(x, y);
            self.buf.get_mut(offset)
        } else {
            None
        }
    }

    // TODO: test this.
    pub fn resize(&mut self, newcols: usize, newrows: usize, value: Cell) {
        let mut newbuf: Vec<Cell> = Vec::with_capacity(newcols * newrows);

        let oldrows = self.rows;
        let oldcols = self.cols;
        let minrows = cmp::min(oldrows, newrows);
        let mincols = cmp::min(oldcols, newcols);
        let x_ext_len = newcols.saturating_sub(oldcols);
        let y_ext_len = newrows.saturating_sub(oldrows) * newcols;

        for y in 0..minrows {
            let copy_start = oldcols * y;
            let copy_end = (oldcols * y) + mincols;

            newbuf.extend_from_slice(&self.buf[copy_start..copy_end]);
            let curlen = newbuf.len();
            newbuf.resize(curlen + x_ext_len, value);
        }

        let curlen = newbuf.len();
        newbuf.resize(curlen + y_ext_len, value);

        self.cols = newcols;
        self.rows = newrows;
        self.buf = newbuf;
    }
}

impl Deref for Panel {
    type Target = [Cell];

    fn deref<'a>(&'a self) -> &'a [Cell] {
        &*self.buf
    }
}

impl DerefMut for Panel {
    fn deref_mut<'a>(&'a mut self) -> &'a mut [Cell] {
        &mut *self.buf
    }
}

impl Index<(usize, usize)> for Panel {
    type Output = Cell;

    fn index<'a>(&'a self, (x, y): (usize, usize)) -> &'a Cell {
        self.get(x, y).expect("index out of bounds")
    }
}

impl IndexMut<(usize, usize)> for Panel {
    fn index_mut<'a>(&'a mut self, (x, y): (usize, usize)) -> &'a mut Cell {
        self.get_mut(x, y).expect("index out of bounds")
    }
}
