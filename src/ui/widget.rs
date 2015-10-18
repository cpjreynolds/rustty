use core::position::{Pos, Size, HasSize, HasPosition};
use core::cellbuffer::{CellAccessor, Cell};
use ui::layout::{Alignable};

trait Widget {
    pub fn draw(&mut self);

    pub fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: usize)
}
