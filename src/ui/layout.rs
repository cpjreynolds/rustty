use core::position::{Pos, Size, HasSize, HasPosition};

pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

pub enum HorizontalAlign {
    Left,
    Middle,
    Right,
}

pub trait Alignable: HasSize + HasPosition {
    fn halign(&mut self, parent: &HasSize, halign: HorizontalAlign, margin: usize) {
        let (cols, _) = self.size();
        let (_, y) = self.origin();
        let (parent_cols, _) = parent.size();
        let newx = match halign {
            HorizontalAlign::Left => margin,
            HorizontalAlign::Right => parent_cols - cols - margin,
            HorizontalAlign::Middle => (parent_cols - cols) / 2,
        };
        self.set_origin((newx, y));
    }

    fn valign(&mut self, parent: &HasSize, valign: VerticalAlign, margin: usize) {
        let (_, rows) = self.size();
        let (x, _) = self.origin();
        let (_, parent_rows) = parent.size();
        let newy = match valign {
            VerticalAlign::Top => margin,
            VerticalAlign::Bottom => parent_rows - rows - margin,
            VerticalAlign::Middle => (parent_rows - rows) / 2,
        };
        self.set_origin((x, newy));
    }

    fn align(&mut self,
             parent: &HasSize,
             halign: HorizontalAlign,
             valign: VerticalAlign,
             margin: usize) {
        self.halign(parent, halign, margin);
        self.valign(parent, valign, margin);
    }
}

pub struct HorizontalLayout<'a> {
    origin: Pos,
    size: Size,
    inner_margin: usize,
    elems: Vec<&'a mut Alignable>,
}

impl<'a> HorizontalLayout<'a> {
    pub fn new(elems: Vec<&mut Alignable>, inner_margin: usize) -> HorizontalLayout {
        let first_origin = elems.first().unwrap().origin();
        let total_width = elems.iter().fold(0, |acc, item| acc + item.size().0);
        let width = total_width + inner_margin * (elems.len() - 1);
        HorizontalLayout {
            origin: first_origin,
            size: (width, 1),
            inner_margin: inner_margin,
            elems: elems,
        }
    }

    pub fn align_elems(&mut self) {
        let (x, y) = self.origin();
        let mut current_x = x;
        for elem in self.elems.iter_mut() {
            elem.set_origin((current_x, y));
            current_x += elem.size().0 + self.inner_margin;
        }
    }
}

impl<'a> HasSize for HorizontalLayout<'a> {
    fn size(&self) -> Size {
        self.size
    }
}

impl<'a> HasPosition for HorizontalLayout<'a> {
    fn origin(&self) -> Pos {
        self.origin
    }

    fn set_origin(&mut self, new_origin: Pos) {
        self.origin = new_origin;
    }
}

impl<'a> Alignable for HorizontalLayout<'a> {}
