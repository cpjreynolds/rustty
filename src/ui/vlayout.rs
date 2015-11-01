use core::position::{Size, Pos, HasSize, HasPosition};
use core::cellbuffer::CellAccessor;
use std::boxed::Box;
use std::collections::HashMap;
use ui::core::{
    Layout, 
    Alignable, 
    HorizontalAlign, 
    VerticalAlign, 
    Widget, 
    Frame, 
    Button,
    Painter,
    ButtonResult
};

/// A special widget that encapsulates buttons and aligns them vertically for
/// drawing within a `Dialog`
///
/// # Examples
///
/// ```
/// use rustty::ui::core::{HorizontalAlign, VerticalAlign, ButtonResult, Button, Widget};
/// use rustty::ui::{Dialog, StdButton, VerticalLayout};
///
/// let mut maindlg = Dialog::new(60, 10);
///
/// let b1 = StdButton::new("Quit", 'q', ButtonResult::Ok);
/// let b2 = StdButton::new("Foo!", 'f', ButtonResult::Custom(1));
/// let b3 = StdButton::new("Bar!", 'b', ButtonResult::Custom(2));
///
/// let buttons = vec![b1, b2, b3].into_iter().map(Box::new);
/// let buttons = buttons.map(|x| x as Box<Button>).collect();
///
/// let mut vlayout = VerticalLayout::from_vec(buttons, 1);
/// vlayout.pack(&maindlg, HorizontalAlign::Middle, VerticalAlign::Bottom, (0,1));
///     
/// maindlg.add_layout(vlayout);
/// ```
///
pub struct VerticalLayout {
    frame: Frame,
    inner_margin: usize,
    origin: Pos,
    widgets: Vec<Box<Button>>
}

impl VerticalLayout {
    /// Construct a `VerticalLayout` object from a vector of boxed objects that implement
    /// [Button](ui/core/button/trait.Button.html). The current API for this function will
    /// change *very* soon
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::ui::core::{Button, ButtonResult};
    /// use rustty::ui::{StdButton, VerticalLayout};
    ///
    /// let b1 = StdButton::new("Quit", 'q', ButtonResult::Ok);
    /// let b2 = StdButton::new("Foo!", 'f', ButtonResult::Custom(1));
    /// let b3 = StdButton::new("Bar!", 'b', ButtonResult::Custom(2));
    ///
    /// let v = vec![b1, b2, b3].into_iter().map(Box::new).map(|x| x as Box<Button>).collect();
    /// let mut hlayout = VerticalLayout::from_vec(v, 1);
    /// ```
    ///
    pub fn from_vec(widgets: Vec<Box<Button>>, inner_margin: usize) -> VerticalLayout {
        let first_origin = widgets.first().unwrap().frame().origin();
        let height = widgets.len() + widgets.len() * inner_margin;
        let width = widgets.iter().map(|s| s.frame().size().0).max().unwrap();
        VerticalLayout {
            frame: Frame::new(width, height),
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

impl Layout for VerticalLayout {
    fn align_elems(&mut self) {
        let (x, y) = self.origin;
        let mut current_y = y;
        for widget in self.widgets.iter_mut() {
            widget.frame_mut().set_origin((x, current_y));
            current_y += 1 + self.inner_margin;
        }
        for w in self.widgets.iter() {
            w.frame().draw_into(&mut self.frame);
        }
    }

    fn forward_keys(&mut self, map: &mut HashMap<char, ButtonResult>) {
       for w in self.widgets.iter() {
            map.insert(w.accel(), w.result());
       }
    }
}
