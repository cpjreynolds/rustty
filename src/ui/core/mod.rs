// Modules HAVE to be public for now or we're going to run into an
// ongoing rust bug, check #18241 to see if they're resolved,
// if so remove pub!
pub mod painter;
pub mod alignable;
pub mod widget;
pub mod button;
pub mod base;
pub mod layout;

// Because of the bug, there's no use showing these re-exports in the docs, 
// so hide them all
#[doc(hidden)]
pub use ui::core::painter::Painter;
#[doc(hidden)]
pub use ui::core::alignable::{Alignable, HorizontalAlign, VerticalAlign};
#[doc(hidden)]
pub use ui::core::layout::Layout;
#[doc(hidden)]
pub use ui::core::widget::Widget;
#[doc(hidden)]
pub use ui::core::base::Base;
#[doc(hidden)]
pub use ui::core::button::{Button, ButtonResult, find_accel_char_index};
