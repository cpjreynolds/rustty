use core::position::{Pos, Size, HasSize, HasPosition};
use core::cellbuffer::{Attr, CellAccessor};
use ui::layout::{Alignable, HorizontalAlign, VerticalAlign};
use ui::widget::Widget;
use ui::painter::Painter;
use ui::base::Base;

#[derive(Clone, Copy)]
pub enum ButtonResult {
    Ok,
    Cancel,
    Custom(i32),
}

fn find_accel_char_index(s: &str, accel: char) -> Option<usize> {
    let lower_accel = accel.to_lowercase().next().unwrap_or(accel);
    for (i, c) in s.chars().enumerate() {
        if c.to_lowercase().next().unwrap_or(c) == lower_accel {
            return Some(i)
        }
    }
    None
}

pub trait Button: Widget {
    fn accel(&self) -> char;
    fn result(&self) -> ButtonResult;
}

pub struct StdButton {
    window: Base,
    accel: char,
    result: ButtonResult
}

impl StdButton {    

    pub fn new(text: &str, accel: char, result: ButtonResult) -> StdButton {
        let s = format!("< {} >", text);
        let width = s.chars().count();
        let mut button = 
            StdButton { window: Base::new(width, 1), 
                        accel: accel.to_lowercase().next().unwrap_or(accel),
                        result: result};
        match find_accel_char_index(text, button.accel) {
            Some(i) => {
                button.window.get_mut(i+2, 0).unwrap().set_attrs(Attr::Bold);
            },
            None    => (),
        }
        button
    }
}

impl Button for StdButton {
    fn accel(&self) -> char {
        self.accel
    }

    fn result(&self) -> ButtonResult {
        self.result
    }
}

impl Widget for StdButton {
    fn draw(&mut self, parent: &mut CellAccessor) {
        self.window.draw_into(parent);
    }

    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: usize) {
        self.window.align(parent, halign, valign, margin);
    }

    fn window(&self) -> &Base {
        &self.window
    }

    fn window_mut(&mut self) -> &mut Base {
        &mut self.window
    }
}

