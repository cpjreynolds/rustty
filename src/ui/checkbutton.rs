use core::position::{Size, HasSize};
use core::cellbuffer::{Attr, CellAccessor};

use ui::core::{
    Alignable,
    HorizontalAlign,
    VerticalAlign,
    Widget,
    Painter,
    Frame,
    Button,
    ButtonResult,
    find_accel_char_index
};

const BALLOT: char = '\u{2610}';
const BALLOT_CHECKED: char = '\u{2611}';

pub struct CheckButton {
    frame: Frame,
    accel: char,
    result: ButtonResult,
    text: String,
    check: char
}

impl CheckButton {
    pub fn new(text: &str, accel: char, result: ButtonResult) -> CheckButton {
        let s = format!("{} {}", BALLOT, text);
        let width = s.chars().count();
        let mut button = 
            CheckButton { frame: Frame::new(width, 1),
                          accel: accel.to_lowercase().next().unwrap_or(accel),
                          result: result,
                          text: text.to_string(),
                          check: BALLOT };
        button.frame.printline(0, 0, &s[..]);
        match find_accel_char_index(text, button.accel) {
            Some(i) => {
                button.frame.get_mut(i+2, 0).unwrap().set_attrs(Attr::Bold);
            },
            None    => (),
        }
        button
    }
}

impl Button for CheckButton {
    fn accel(&self) -> char {
        self.accel
    }

    fn result(&self) -> ButtonResult {
        self.result
    }

    fn pressed(&mut self) {
        self.check = 
            if self.check == BALLOT {
                BALLOT_CHECKED
            } else {
                BALLOT
            };

        let s = format!("{} {}", self.check, self.text);
        self.frame.printline(0, 0, &s[..]);
    }

    fn state(&self) -> bool {
        self.check == BALLOT_CHECKED
    }
}

impl Widget for CheckButton {
    fn draw(&mut self, parent: &mut CellAccessor) {
        self.frame.draw_into(parent);
    }

    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
            margin: (usize, usize)) {
        self.frame.align(parent, halign, valign, margin);
    }

    fn draw_box(&mut self) {
        self.frame.draw_box();
    }

    fn resize(&mut self, new_size: Size) {
        self.frame.resize(new_size);
    }

    fn frame(&self) -> &Frame {
        &self.frame
    }

    fn frame_mut(&mut self) -> &mut Frame {
        &mut self.frame
    }
}


