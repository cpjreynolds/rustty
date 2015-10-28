use core::position::HasSize;
use core::cellbuffer::CellAccessor;
use ui::core::alignable::{HorizontalAlign, VerticalAlign};
use ui::core::frame::Frame;

/// Every UI element will inherit from trait, widgets are the foundation of
/// UI, thus every drawable widget will implement some way to draw, align and
/// return the renderer (Base in most cases)
pub trait Widget {
    /// Draws the widget to the valid `CellAccessor` passed
    fn draw(&mut self, parent: &mut CellAccessor);

    /// Aligns the widget with the `parent` as reference
    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: (usize, usize));

    fn draw_box(&mut self);
    
    /// Return a reference the renderer, `Base` in general cases
    fn frame(&self) -> &Frame;
    /// Return a mutable reference to the renderer, `Base` in general cases
    fn frame_mut(&mut self) -> &mut Frame;
}
