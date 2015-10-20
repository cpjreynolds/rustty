use core::position::HasSize;
use core::cellbuffer::CellAccessor;
use ui::core::layout::{HorizontalAlign, VerticalAlign};
use ui::core::base::Base;

pub trait Widget {
    fn draw(&mut self, parent: &mut CellAccessor);

    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: usize);

    fn window(&self) -> &Base;
    fn window_mut(&mut self) -> &mut Base;
}
