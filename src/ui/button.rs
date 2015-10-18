use core::position::{Pos, Size, HasSize, HasPosition};
use core::cellbuffer::{Attr, CellAccessor};
use ui::layout::{Alignable, HorizontalAlign, VerticalAlign};
use ui::widget::Widget;
use ui::painter::Painter;
use ui::base::Base;

#[derive(Clone, Copy)]
pub enum ButtonResult {
    Od,
    Cancel,
    Custom(i32),
}

pub trait Button: Widget {
    fn find_accel_char_index(s: &str, accel: char) -> Option<usize> {
        let lower_accel = accel.to_lowercase().next().unwrap_or(accel);
        for (i, c) in s.chars().enumerate() {
            if c.to_lowercase().next().unwrap_or(c) == lower_accel {
                return Some(i)
            }
        }
        None
    }

    fn accel(&self) -> char;
    fn result(&self) -> ButtonResult;
}

pub struct StdButton {
    window: Base,
    accel: Option<char>,
    result: ButtonResult
}

impl StdButton {    

    pub fn new(text: &str, accel: Option<char>, result: ButtonResult) -> StdButton {
        let s = format!("< {} >", text);
        let width = s.chars().count();
        let mut button = 
            StdButton { window: Base::new(width, 1), 
                        accel: accel,
                        result: result};
        match accel {
            Some(c) => {
                match Button::find_accel_char_index(text, c) {
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

impl Button for StdButton {
    fn accel(&self) -> char {
        return self.accel.unwrap();
    }

    fn result(&self) -> ButtonResult {
        return self.result;
    }
}

impl Widget for StdButton {
    pub fn draw(&mut self, parent: &mut HasSize) {
        self.window.draw_into(&mut parent);
    }

    pub fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: usize) {
        self.align(&parent, halign, valign, margin);
    }
}

