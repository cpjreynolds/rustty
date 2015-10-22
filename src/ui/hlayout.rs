use ui::core::{Alignable, Widget, Base};
use core::position::{Pos, Size, HasSize, HasPosition};
use std::boxed::Box;

pub struct HorizontalLayout {
    frame: Option<Base>,
    inner_margin: usize,
    size: Size,
    origin: Pos,
    elems: Vec<Box<Widget>>
}

impl HorizontalLayout {
    pub fn new(inner_margin: usize) -> HorizontalLayout {
        HorizontalLayout {
            base: None,
            origin: (0,0),
            size: (0,0),
            inner_margin: inner_margin,
            elems: Vec::new()
        }
    }

    pub fn add_widget<T: Widget + 'static>(&mut self, widget: T) {
        self.elems.push(Box::new(widget));
    }

    pub fn align_elems(&mut self) {
        let (x, y) = self.elems.first().unwrap().window().origin();
        let total_width = self.elems.iter().fold(0, |acc, item| 
                                                 acc + item.window_mut().size().0);
        let width = total_width + self.inner_margin * (elemns.len() - 1);
        self.frame = Base::new(width, 1);
        let mut current_x = x;
        for elem in self.elems.iter_mut() {
            elem.window_mut().set_origin((current_x, y));
            current_x += elem.window_mut().size().0 + self.inner_margin;
        }
    }
}

impl HasSize for HorizontalLayout {
    fn size(&self) -> Size { 
        self.size
    }
}

impl HasPosition for HorizontalLayout {
    fn origin(&self) -> Pos {
        self.origin
    }

    fn set_origin(&mut self, new_origin: Pos)  {
        self.origin = new_origin;
    }
}

impl Alignable for HorizontalLayout {}
/*
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

pub struct VerticalLayout<'a> {
    origin: Pos,
    size: Size,
    elems: Vec<&'a mut Alignable>,
}

impl <'a> VerticalLayout<'a> {
    pub fn new(elems: Vec<&mut Alignable>) -> VerticalLayout {
        let first_origin = elems.first().unwrap().origin();
        let height = elems.len();
        let width = elems.iter().map(|s| s.size().0).max().unwrap();
        VerticalLayout {
            origin: first_origin,
            size: (width, height),
            elems: elems,
        }
    }

    pub fn align_elems(&mut self) {
        let (x, y) = self.origin();
        let mut current_y = y;
        for elem in self.elems.iter_mut() {
            elem.set_origin((x, current_y));
            current_y += 1;
        }
    }
}

impl<'a> HasSize for VerticalLayout<'a> {
    fn size(&self) -> Size {
        self.size
    }
}

impl<'a> HasPosition for VerticalLayout<'a> {
    fn origin(&self) -> Pos {
        self.origin
    }

    fn set_origin(&mut self, new_origin: Pos) {
        self.origin = new_origin;
    }
}

impl<'a> Alignable for VerticalLayout<'a> {}
*/
