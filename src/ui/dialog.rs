use std::collections::HashMap;
use std::boxed::Box;

use core::position::{Size, HasSize};
use core::cellbuffer::CellAccessor;

use ui::core::{
    Alignable,
    HorizontalAlign,
    VerticalAlign,
    Widget,
    Base,
    Button,
    ButtonResult
};

pub struct Dialog {
    window: Base,
    buttons: Vec<Box<Button>>,
    accel2result: HashMap<char, ButtonResult>,
}


impl Dialog {
    pub fn new(cols: usize, rows: usize) -> Dialog {
        Dialog {
            window: Base::new(cols, rows),
            buttons: Vec::new(),
            accel2result: HashMap::new(),
        }
    }

    pub fn add_button<T: Button + 'static>(&mut self, button: T) {
        self.accel2result.insert(button.accel(), button.result());
        self.buttons.push(Box::new(button));

        self.buttons.last_mut().unwrap().window().draw_into(&mut self.window);
    }

    pub fn result_for_key(&self, key: char) -> Option<ButtonResult> {
        match self.accel2result.get(&key.to_lowercase().next().unwrap_or(key)) {
            Some(r) => Some(*r),
            None => None,
        }
    }
}

impl Widget for Dialog {
    fn draw(&mut self, parent: &mut CellAccessor) {
        self.window.draw_into(parent);
    }
    
    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: usize) {
        self.window_mut().align(parent, halign, valign, margin);
    }

    fn window(&self) -> &Base {
        &self.window
    }

    fn window_mut(&mut self) -> &mut Base {
        &mut self.window
    }
}

impl HasSize for Dialog {
    fn size(&self) -> Size {
        self.window.size()
    }
}
