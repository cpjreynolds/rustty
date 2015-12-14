use core::position::{Size, HasSize};
use core::cellbuffer::CellAccessor;
use ui::core::attributes::{HorizontalAlign, VerticalAlign};
use ui::core::frame::Frame;

/// Widgets are the foundation of UI, all frontend objects inherit the widget
/// type and are generalized through either the widget itself or a specialized
/// widget (e.g. *Button*, *Layout*). 
pub trait Widget {
    /// Draws the widget to the valid `CellAccessor` passed
    fn draw(&mut self, parent: &mut CellAccessor);

    /// Aligns the widget with the `parent` as reference
    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: (usize, usize));

    /// Expose the painter trait `draw_box` for all widgets, which outlines
    /// the space enclosed within the widget
    fn draw_box(&mut self);

    /// Resize the given widget to new dimensions given by `Size`
    fn resize(&mut self, new_size: Size);

    /// Return a reference the renderer, `Base` in general cases
    fn frame(&self) -> &Frame;

    /// Return a mutable reference to the renderer, `Base` in general cases
    fn frame_mut(&mut self) -> &mut Frame;
}
