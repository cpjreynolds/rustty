use core::position::{Pos, Size, HasSize, HasPosition};
use core::cellbuffer::{CellAccessor, Cell};
use ui::core::layout::{Alignable, HorizontalAlign, VerticalAlign};
use ui::core::base::Base;

pub trait Widget {
    fn draw(&mut self, parent: &mut CellAccessor);

    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: usize);

    fn window(&self) -> &Base;
    fn window_mut(&mut self) -> &mut Base;
}
