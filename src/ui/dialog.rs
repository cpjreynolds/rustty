use std::collections::HashMap;

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
use ui::button::{create_button};


pub struct Dialog {
    window: Base,
    buttons: Vec<Widget>,
    accel2result: HashMap<char, DialogResult>,
}

impl Widget for Dialog {
    pub fn draw(&mut self, parent: &HasSize) {
        self.window.draw_into(&mut parent);
    }
    /*
    pub fn draw(&mut self) {
        let button_count = self.buttons.len();
        self.draw_buttons_subset(0, button_count, layout);
    }

    pub fn draw_subset(&mut self, i: usize, u: usize) {
        fn f(b: &mut Widget) -> &mut Alignable { &mut *b }
        {
            let elems = self.buttons[i..u].iter_mut().map(f).collect();
            match layout {
                ButtonLayout::Vertical(g)   => {
                    let mut l = VerticalLayout::new(elems);
                    l.align(&self.window, g, VerticalAlign::Bottom, 1);
                    l.align_elems();
                },
                ButtonLayout::Horizontal(i) => {
                    let mut l = HorizontalLayout::new(elems, 2);
                    l.align(&self.window, HorizontalAlign::Middle, VerticalAlign::Bottom, i);
                    l.align_elems();
                }
            }
        }
        for b in self.buttons[i..u].iter() {
            b.draw_into(&mut self.window);
        }
    }
    */

    pub fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: usize) {
        self.align(&parent, halign, valign, margin);`
    }
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
    }

    /*
    pub fn add_button(&mut self, text: &str, accel: char, result: DialogResult) -> &mut Widget {
        let widget = create_button(text, Some(accel));
        self.accel2result.insert(accel.to_lowercase().next().unwrap_or(accel), result);
        self.buttons.push(widget);
        self.buttons.last_mut().unwrap()
    }
    */

    pub fn result_for_key(&self, key: char) -> Option<DialogResult> {
        match self.accel2result.get(&key.to_lowercase().next().unwrap_or(key)) {
            Some(r) => Some(*r),
            None => None,
        }
    }
}
