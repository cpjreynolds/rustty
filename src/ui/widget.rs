use core::position::{Pos, Size, HasSize, HasPosition};
use core::cellbuffer::{CellAccessor, Cell};
use ui::layout::{Alignable, HorizontalAlign, VerticalAlign};

pub trait Widget {
    fn draw(&mut self);

    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: usize);
}
