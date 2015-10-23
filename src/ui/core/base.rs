use core::position::{Pos, Size, HasSize, HasPosition};
use core::cellbuffer::{CellAccessor, Cell};
use ui::core::alignable::{Alignable};

/// The `Base` struct is the building block for all future
/// widgets inside of *ui*. Objects of `Base` abstract away
/// the actual creation and drawing of areas of a terminal,
/// because this process is the same for all widgets. Every
/// widget should contain one `Base` type to be used to render
/// text to the screen
pub struct Base {
    origin: Pos,
    size: Size,
    buf: Vec<Cell>,
}

impl Base {
    /// Constructs a new Base object with a width of `cols`
    /// and height of `rows`
    pub fn new(cols: usize, rows: usize) -> Base {
        Base {
            origin: (0, 0),
            size: (cols, rows),
            buf: vec![Cell::default(); cols * rows],
        }
    }

    /// Draw the buffer contained inside of the base object to 
    /// a valid object that implements CellAccessor.
    pub fn draw_into(&self, cells: &mut CellAccessor) {
        let (cols, rows) = self.size();
        let (x, y) = self.origin();
        for ix in 0..cols {
            let offset_x = x + ix;
            for iy in 0..rows {
                let offset_y = y + iy;
                match cells.get_mut(offset_x, offset_y) {
                    Some(cell) => { *cell = *self.get(ix, iy).unwrap(); },
                    None => (),
                }
            }
        }
    }
}

impl HasSize for Base {
    fn size(&self) -> Size {
        self.size
    }
}

impl CellAccessor for Base {
    fn cellvec(&self) -> &Vec<Cell> {
        &self.buf
    }

    fn cellvec_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.buf
    }

}

impl HasPosition for Base {
    fn origin(&self) -> Pos {
        self.origin
    }

    fn set_origin(&mut self, new_origin: Pos) {
        self.origin = new_origin;
    }
}

impl Alignable for Base {}
