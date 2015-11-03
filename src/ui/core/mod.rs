// Modules HAVE to be public for now or we're going to run into an
// ongoing rust bug, check #18241 to see if they're resolved,
// if so remove pub!
pub mod painter;
pub mod alignable;
pub mod widget;
pub mod button;
pub mod frame;
pub mod layout;
pub mod attributes;

// Because of the bug, there's no use showing these re-exports in the docs, 
// so hide them all
#[doc(hidden)]
pub use ui::core::painter::Painter;
#[doc(hidden)]
pub use ui::core::alignable::Alignable;
#[doc(hidden)]
pub use ui::core::layout::Layout;
#[doc(hidden)]
pub use ui::core::widget::Widget;
#[doc(hidden)]
pub use ui::core::frame::Frame;
#[doc(hidden)]
pub use ui::core::button::{Button, find_accel_char_index};
#[doc(hidden)]
pub use ui::core::attributes::{ButtonResult, HorizontalAlign, VerticalAlign, Resizable};
