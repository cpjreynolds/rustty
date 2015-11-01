use core::position::{Size, HasSize};
use core::cellbuffer::CellAccessor;

use ui::core::{
    Alignable,
    HorizontalAlign,
    VerticalAlign,
    Widget,
    Frame,
    Painter
};

pub struct Label {
    frame: Frame,
    text: String,
    x: usize,
    y: usize,
    t_halign: HorizontalAlign,
    t_valign: VerticalAlign,
    t_margin: (usize, usize)
}

impl Label {
    pub fn new(cols: usize, rows: usize) -> Label {
        Label {
            frame: Frame::new(cols, rows),
            text: "".to_string(),
            x: 0,
            y: 0,
            t_halign: HorizontalAlign::Left,
            t_valign: VerticalAlign::Top,
            t_margin: (1, 1)
        }
    }

    pub fn from_str(s: String) -> Label {
        Label {
            frame: Frame::new(s.len(), 1),
            text: s,
            x: 0,
            y: 0,
            t_halign: HorizontalAlign::Left,
            t_valign: VerticalAlign::Top,
            t_margin: (1, 1)
        }
    }

    pub fn align_text(&mut self, halign: HorizontalAlign, valign: VerticalAlign,
                    margin: (usize, usize)) {
        self.t_halign = halign.clone();
        self.t_valign = valign.clone();
        self.t_margin = margin;

        self.x = self.frame.halign_line(&self.text, halign, margin.0);
        self.y = self.frame.valign_line(&self.text, valign, margin.1);
    }

    pub fn set_text(&mut self, new_str: String) {
        self.text = new_str;
        let (framex, framey) = self.frame.size();
        if self.text.len() > (framex * framey) {
            self.frame.resize((self.text.len() - framex * framey, framey));
        }
    }
}

impl Widget for Label {
    fn draw(&mut self, parent: &mut CellAccessor) {
        self.frame.printline(self.x, self.y, &self.text);
        self.frame.draw_into(parent);
    }

    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: (usize, usize)) {
        self.frame.align(parent, halign, valign, margin);
        self.x = self.frame.halign_line(&self.text, self.t_halign.clone(), margin.0);
        self.y = self.frame.valign_line(&self.text, self.t_valign.clone(), margin.1);
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
