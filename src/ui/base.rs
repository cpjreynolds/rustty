use core::position::{Pos, Size, HasSize, HasPosition};
use core::cellbuffer::{CellAccessor, Cell};
use ui::layout::{Alignable};
use ui::widget::Widget;

pub struct Base {
    origin: Pos,
    size: Size,
    buf: Vec<Cell>,
}

impl Base {
    pub fn new(cols: usize, rows: usize) -> Base {
        Base {
            origin: (0, 0),
            size: (cols, rows),
            buf: vec![Cell::default(); cols * rows],
        }
    }

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
