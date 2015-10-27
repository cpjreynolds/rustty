use ui::core::{Layout, Alignable, HorizontalAlign, VerticalAlign, Widget, Base, Button, ButtonResult};
use core::position::{Pos, HasSize, HasPosition};
use core::cellbuffer::CellAccessor;
use std::boxed::Box;
use std::collections::HashMap;

pub struct HorizontalLayout {
    frame: Base,
    inner_margin: usize,
    origin: Pos,
    widgets: Vec<Box<Button>>
}

impl HorizontalLayout {
    pub fn from_vec(widgets: Vec<Box<Button>>, inner_margin: usize) -> HorizontalLayout {
        let first_origin = widgets.first().unwrap().window().origin();
        let total_width = widgets.iter().fold(0, |acc, item| acc + item.window().size().0);
        let width = total_width + inner_margin * (widgets.len() - 1);
        HorizontalLayout {
            frame: Base::new(width, 1),
            inner_margin: inner_margin,
            origin: first_origin,
            widgets: widgets
        }
    }

}

impl Widget for HorizontalLayout {
    fn draw(&mut self, parent: &mut CellAccessor) { 
        self.frame.draw_into(parent);
    }

    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: (usize, usize)) {
        self.frame.align(parent, halign, valign, margin);
    }

    fn window(&self) -> &Base {
        &self.frame
    }

    fn window_mut(&mut self) -> &mut Base {
        &mut self.frame
    }
}

impl Layout for HorizontalLayout {
    fn align_elems(&mut self) {
        let (x, y) = self.origin;
        let mut current_x = x;
        for widget in self.widgets.iter_mut() {
            widget.window_mut().set_origin((current_x, y));
            current_x += widget.window_mut().size().0 + self.inner_margin;
        }
        for w in self.widgets.iter() {
            w.window().draw_into(&mut self.frame);
        }
    }

    fn forward_keys(&mut self, map: &mut HashMap<char, ButtonResult>) {
       for w in self.widgets.iter() {
            map.insert(w.accel(), w.result());
       }
    }
}
