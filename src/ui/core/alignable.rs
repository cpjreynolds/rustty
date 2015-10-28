use core::position::{HasSize, HasPosition};

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
    
    fn align(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign, 
             margin: (usize, usize)) {
        self.halign(parent, halign, margin.0);
        self.valign(parent, valign, margin.1);
    }
}

