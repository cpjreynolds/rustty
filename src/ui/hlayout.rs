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

/// Hold buttons and align them horizontally for drawing within a 
/// [Dialog](struct.Dialog.html)
///
/// # Examples
///
/// ```
/// use rustty::ui::core::{HorizontalAlign, VerticalAlign, ButtonResult, Widget, Button};
/// use rustty::ui::{Dialog, StdButton, HorizontalLayout};
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
/// let mut hlayout = HorizontalLayout::from_vec(buttons, 1);
/// hlayout.pack(&maindlg, HorizontalAlign::Middle, VerticalAlign::Bottom, (0,1));
///     
/// maindlg.add_layout(hlayout);
/// ```
///
pub struct HorizontalLayout {
    frame: Frame,
    inner_margin: usize,
    origin: Pos,
    widgets: Vec<Box<Button>>
}

impl HorizontalLayout {
    /// Construct a `HorizontalLayout` object from a vector of boxed objects that implement
    /// [Button](core/button/trait.Button.html). The current API for this function will
    /// change *very* soon
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::ui::core::{ButtonResult, Button};
    /// use rustty::ui::{StdButton, HorizontalLayout};
    ///
    /// let b1 = StdButton::new("Quit", 'q', ButtonResult::Ok);
    /// let b2 = StdButton::new("Foo!", 'f', ButtonResult::Custom(1));
    /// let b3 = StdButton::new("Bar!", 'b', ButtonResult::Custom(2));
    ///
    /// let v = vec![b1, b2, b3].into_iter().map(Box::new).map(|x| x as Box<Button>).collect();
    /// let mut hlayout = HorizontalLayout::from_vec(v, 1);
    /// ```
    ///
    pub fn from_vec(widgets: Vec<Box<Button>>, inner_margin: usize) -> HorizontalLayout {
        let first_origin = widgets.first().unwrap().frame().origin();
        let total_width = widgets.iter().fold(0, |acc, item| acc + item.frame().size().0);
        let width = total_width + inner_margin * (widgets.len() - 1);
        HorizontalLayout {
            frame: Frame::new(width, 1),
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

impl Layout for HorizontalLayout {
    fn align_elems(&mut self) {
        let (x, y) = self.origin;
        let mut current_x = x;
        for widget in self.widgets.iter_mut() {
            widget.frame_mut().set_origin((current_x, y));
            current_x += widget.frame_mut().size().0 + self.inner_margin;
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
