use core::cellbuffer::{Attr, CellAccessor};
use ui::widget::Widget;
use ui::painter::Painter;
use ui::base::Base;

#[derive(Clone, Copy)]
pub enum ButtonResult {
    Ok,
    Cancel,
    Custom(i32),
}

trait Button: Widget {
    fn find_accel_char_index(s: &str, accel: char) -> Option<usize> {
        let lower_accel = accel.to_lowercase().next().unwrap_or(accel);
        for (i, c) in s.chars().enumerate() {
            if c.to_lowercase().next().unwrap_or(c) == lower_accel {
                return Some(i)
            }
        }
        None
    }

    fn accel() -> char;
    fn result() -> ButtonResult;
}

struct StdButton {
    window: Base,
    accel: Option<char>
}

impl StdButton {
    fn new(text: &str, accel: Option<char>) -> StdButton {
        let s = format!("< {} >", text);
        let width = s.chars().count();
        let mut button = StdButton { window: Base::new(width, 1), accel: None };
        match accel {
            Some(c) => {
                match find_accel_char_index(text, c) {
                    Some(i) => {
                        button.window.get_mut(i+2, 0).unwrap().set_attrs(Attr::Bold);
                    },
                    None    => (),
                }
            },
            None    => ()
        }
        button
    }
}

impl Button for StdButton { }

impl Widget for StdButton {
    pub fn draw(&mut self, parent: &mut HasSize) {
        self.window.draw_into(&mut HasSize);
    }

    pub fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: usize) {
        self.align(&parent, halign, valign, margin);
    }
}

