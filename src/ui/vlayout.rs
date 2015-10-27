use ui::core::{Layout, Alignable, HorizontalAlign, VerticalAlign, Widget, Base, Button, ButtonResult};
use core::position::{Pos, HasSize, HasPosition};
use core::cellbuffer::CellAccessor;
use std::boxed::Box;
use std::collections::HashMap;

pub struct VerticalLayout {
    frame: Base,
    inner_margin: usize,
    origin: Pos,
    widgets: Vec<Box<Button>>
}

impl VerticalLayout {
    pub fn from_vec(widgets: Vec<Box<Button>>, inner_margin: usize) -> VerticalLayout {
        let first_origin = widgets.first().unwrap().window().origin();
        let height = widgets.len() + widgets.len() * inner_margin;
        let width = widgets.iter().map(|s| s.window().size().0).max().unwrap();
        VerticalLayout {
            frame: Base::new(width, height),
            inner_margin: inner_margin,
            origin: first_origin,
            widgets: widgets
        }
    }

}

impl Widget for VerticalLayout {
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

impl Layout for VerticalLayout {
    fn align_elems(&mut self) {
        let (x, y) = self.origin;
        let mut current_y = y;
        for widget in self.widgets.iter_mut() {
            widget.window_mut().set_origin((x, current_y));
            current_y += 1 + self.inner_margin;
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
