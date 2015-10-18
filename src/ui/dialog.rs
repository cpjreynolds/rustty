use std::collections::HashMap;

use core::position::{HasSize};
use ui::layout::{
    Alignable,
    HorizontalLayout,
    HorizontalAlign,
    VerticalAlign,
    VerticalLayout,
    ButtonLayout,
};

use ui::widget::Widget;
use ui::base::Base;
use ui::button::{Button, ButtonResult};


pub struct Dialog {
    window: Base,
    buttons: Vec<Button>,
    accel2result: HashMap<char, ButtonResult>,
}


impl Dialog {
    pub fn new(cols: usize, rows: usize) -> Dialog {
        Dialog {
            window: Widget::new(cols, rows),
            buttons: Vec::new(),
            accel2result: HashMap::new(),
        }
    }

    pub fn window(&self) -> &Widget {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut Widget {
        &mut self.window
    }

    pub fn add_button(&mut self, button: Button) {
        self.accel2result.insert(button.accel(), button.result());
        self.buttons.push(button);

        self.buttons.last_mut().draw_into(&mut self.window);
    }

    pub fn result_for_key(&self, key: char) -> Option<ButtonResult> {
        match self.accel2result.get(&key.to_lowercase().next().unwrap_or(key)) {
            Some(r) => Some(*r),
            None => None,
        }
    }
}

impl Widget for Dialog {
    fn draw(&mut self, parent: &HasSize) {
        self.window.draw_into(&mut parent);
    }
    
    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: usize) {
        self.align(&parent, halign, valign, margin);
    }
}
