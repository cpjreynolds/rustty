use std::collections::HashMap;
use std::boxed::Box;

use core::position::{Size, HasSize};
use core::cellbuffer::CellAccessor;

use ui::core::{
    Alignable,
    HorizontalAlign,
    VerticalAlign,
    Widget,
    Base,
    Button,
    ButtonResult,
    Layout
};

/// A Widget that can bind buttons and display text.
///
/// # Examples
///
/// ```
/// use rustty::ui::core::{VerticalAlign, HorizontalAlign, ButtonResult, Widget, Painter};
/// use rustty::ui::{Dialog, StdButton};
///
/// let mut maindlg = Dialog::new(60, 10);
///
/// let mut b1 = StdButton::new("Quit", 'q', ButtonResult::Ok);
/// b1.pack(&maindlg, HorizontalAlign::Left, VerticalAlign::Middle, (1,1));
///
/// maindlg.window_mut().draw_box();
/// // draw to terminal
/// // maindlg.window.draw_into(&mut term);
/// ```
///
pub struct Dialog {
    window: Base,
    buttons: Vec<Box<Button>>,
    layouts: Vec<Box<Layout>>,
    accel2result: HashMap<char, ButtonResult>,
}


impl Dialog {
    /// Construct a new Dialog widget `cols` wide by `rows` high.
    /// 
    /// # Examples
    ///
    /// ```
    /// use rustty::ui::Dialog;
    ///
    /// let mut maindlg = Dialog::new(60, 10);
    /// ```
    pub fn new(cols: usize, rows: usize) -> Dialog {
        Dialog {
            window: Base::new(cols, rows),
            buttons: Vec::new(),
            layouts: Vec::new(),
            accel2result: HashMap::new(),
        }
    }

    /// Add an existing widget that implements the [Button](ui/core/button/trait.Button.html)
    /// trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::ui::core::{Widget, ButtonResult, HorizontalAlign, VerticalAlign};
    /// use rustty::ui::{Dialog, StdButton};
    /// let mut maindlg = Dialog::new(60, 10);
    ///
    /// let mut b1 = StdButton::new("Quit", 'q', ButtonResult::Ok);
    /// b1.pack(&maindlg, HorizontalAlign::Middle, VerticalAlign::Middle, (1,1));
    /// maindlg.add_button(b1);
    /// ```
    pub fn add_button<T: Button + 'static>(&mut self, button: T) {
        self.accel2result.insert(button.accel(), button.result());
        self.buttons.push(Box::new(button));

        self.buttons.last_mut().unwrap().window().draw_into(&mut self.window);
    }

    /// Add an existing widget that implements the [Layout](ui/core/layout/trait.Layout.html)
    /// trait. **NEEDS A REWORK**, the way of passing in a vector of buttons is ugly and a 
    /// very bad API.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::ui::core::{HorizontalAlign, VerticalAlign, ButtonResult, Button, Widget};
    /// use rustty::ui::{Dialog, StdButton, VerticalLayout};
    ///
    /// let mut maindlg = Dialog::new(60, 10);
    /// let b1 = StdButton::new("Foo", 'f', ButtonResult::Ok);
    /// let b2 = StdButton::new("Bar", 'b', ButtonResult::Cancel);
    ///
    /// let vec = vec![b1, b2].into_iter().map(Box::new).map(|x| x as Box<Button>).collect();
    /// let mut vlayout = VerticalLayout::from_vec(vec, 0);
    /// vlayout.pack(&maindlg, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));
    ///
    /// maindlg.add_layout(vlayout);
    /// ```
    ///
    pub fn add_layout<T: Layout + 'static>(&mut self, layout: T) {
        self.layouts.push(Box::new(layout));
        
        self.layouts.last_mut().unwrap().align_elems();
        self.layouts.last_mut().unwrap().window().draw_into(&mut self.window);
        self.layouts.last_mut().unwrap().forward_keys(&mut self.accel2result);
    }

    /// Checks whether the char passed is a valid key for any buttons currently
    /// drawn within the dialog, if so the corresponding `ButtonResult` is returned
    pub fn result_for_key(&self, key: char) -> Option<ButtonResult> {
        match self.accel2result.get(&key.to_lowercase().next().unwrap_or(key)) {
            Some(r) => Some(*r),
            None => None,
        }
    }
}

impl Widget for Dialog {
    fn draw(&mut self, parent: &mut CellAccessor) {
        self.window.draw_into(parent);
    }
    
    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: (usize, usize)) {
        self.window_mut().align(parent, halign, valign, margin);
    }

    fn window(&self) -> &Base {
        &self.window
    }

    fn window_mut(&mut self) -> &mut Base {
        &mut self.window
    }
}

impl HasSize for Dialog {
    fn size(&self) -> Size {
        self.window.size()
    }
}
