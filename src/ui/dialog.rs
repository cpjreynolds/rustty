use std::collections::HashMap;

use ui::layout::{Alignable, HorizontalLayout, HorizontalAlign, VerticalAlign};
use ui::widget::Widget;
use ui::button::create_button;

#[derive(Clone, Copy)]
pub enum DialogResult {
    Ok,
    Cancel,
    Custom(i32),
}

pub struct Dialog {
    window: Widget,
    buttons: Vec<Widget>,
    accel2result: HashMap<char, DialogResult>,
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

    pub fn add_button(&mut self, text: &str, accel: char, result: DialogResult) -> &mut Widget {
        let widget = create_button(text, Some(accel));
        self.accel2result.insert(accel.to_lowercase().next().unwrap_or(accel), result);
        self.buttons.push(widget);
        self.buttons.last_mut().unwrap()
    }

    pub fn result_for_key(&self, key: char) -> Option<DialogResult> {
        match self.accel2result.get(&key.to_lowercase().next().unwrap_or(key)) {
            Some(r) => Some(*r),
            None => None,
        }
    }

    pub fn draw_buttons(&mut self) {
        fn f(b: &mut Widget) -> &mut Alignable {
            &mut *b
        }
        {
            let elems = self.buttons.iter_mut().map(f).collect();
            let mut l = HorizontalLayout::new(elems, 2);
            l.align(&self.window,
                    HorizontalAlign::Middle,
                    VerticalAlign::Bottom,
                    1);
            l.align_elems();
        }
        for b in self.buttons.iter() {
            b.draw_into(&mut self.window);
        }
    }
}
