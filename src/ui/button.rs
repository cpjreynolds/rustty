use core::cellbuffer::{Attr, CellAccessor};
use ui::widget::Widget;
use ui::painter::Painter;

fn find_accel_char_index(s: &str, accel: char) -> Option<usize> {
    let lower_accel = accel.to_lowercase().next().unwrap_or(accel);
    for (i, c) in s.chars().enumerate() {
        if c.to_lowercase().next().unwrap_or(c) == lower_accel {
            return Some(i);
        }
    }
    None
}

pub fn create_button(text: &str, accel: Option<char>) -> Widget {
    let s = format!("< {} >", text);
    let width = s.chars().count();
    let mut widget = Widget::new(width, 1);
    widget.printline(0, 0, &s[..]);
    match accel {
        Some(c) => {
            match find_accel_char_index(text, c) {
                Some(i) => {
                    widget.get_mut(i + 2, 0).unwrap().set_attrs(Attr::Bold);
                }
                None => (),
            }
        }
        None => (),
    }
    widget
}
