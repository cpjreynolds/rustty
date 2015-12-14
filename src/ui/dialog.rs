use std::collections::HashMap;
use std::boxed::Box;

use core::position::{Size, HasSize};
use core::cellbuffer::CellAccessor;

use ui::core::{
    Alignable,
    HorizontalAlign,
    VerticalAlign,
    Widget,
    Frame,
    Button,
    ButtonResult,
    Layout,
    Painter
};

use ui::label::Label;

/// Pack labels, buttons and other widgets into dialogs
///
/// # Examples
///
/// ```
/// use rustty::ui::core::{VerticalAlign, HorizontalAlign, ButtonResult, Widget};
/// use rustty::ui::{Dialog, StdButton};
///
/// let mut maindlg = Dialog::new(60, 10);
///
/// let mut b1 = StdButton::new("Quit", 'q', ButtonResult::Ok);
/// b1.pack(&maindlg, HorizontalAlign::Left, VerticalAlign::Middle, (1,1));
///
/// maindlg.draw_box();
/// // draw to terminal
/// // maindlg.draw(&mut term);
/// ```
///
pub struct Dialog {
    frame: Frame,
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
            frame: Frame::new(cols, rows),
            buttons: Vec::new(),
            layouts: Vec::new(),
            accel2result: HashMap::new(),
        }
    }

    /// Add an existing widget that implements the [Button](core/button/trait.Button.html)
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

        self.buttons.last_mut().unwrap().draw(&mut self.frame);
    }

    /// Add an existing widget that implements the [Layout](core/layout/trait.Layout.html)
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
        self.layouts.last_mut().unwrap().frame().draw_into(&mut self.frame);
        self.layouts.last_mut().unwrap().forward_keys(&mut self.accel2result);
    }

    /// Add an existing label that contains some text.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::ui::core::{HorizontalAlign, VerticalAlign, Widget};
    /// use rustty::ui::{Dialog, Label};
    ///
    /// let mut maindlg = Dialog::new(60, 10);
    /// let mut lbl = Label::from_str("Foo");
    /// lbl.pack(&maindlg, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));
    ///
    /// maindlg.add_label(lbl);
    /// ```
    ///
    pub fn add_label(&mut self, mut label: Label) {
        label.draw(&mut self.frame);
    }

    /// Change the state of an existing CheckButton, if any exists, 
    /// within the dialog. If an invalid button result is given, the
    /// function will panic. *Note* StdButtons are still valid handles
    /// for this function, however they will not actually do anything.
    /// This function is for buttons that perform some action upon being
    /// pressed.
    ///
    /// # Examples
    /// 
    /// ```ignore
    /// // match character for dialog
    /// match dialog.result_for_key(ch) {
    ///     Some(ButtonResult::Ok)  => {
    ///         dialog.button_pressed(ButtonResult::Custom(i));
    ///         // do stuff ...
    ///     },
    ///     _                       => { }
    /// }
    /// ```
    ///
    pub fn button_pressed(&mut self, res: ButtonResult) {
        match self.buttons.iter_mut().find(|x| x.result() == res) {
            Some(i) => { i.pressed(); i.draw(&mut self.frame)}
            _       => { panic!("Not a valid button result for\
                                Dialog::button_checked()"); }
        }
    }

    /// For buttons that have a state manager, this function will return
    /// the current state of the button. *CheckButton* for example uses 
    /// a state to manage whether the button is checked, different actions
    /// can be taken depending on the state once read.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // match character for dialog
    /// match dialog.result_for_key(ch) {
    ///     Some(ButtonResult::Ok)  => {
    ///         dialog.button_pressed(ButtonResult::Custom(i));
    ///         if dialog.is_button_pressed(ButtonResult::Custom(i)) {
    ///             // do ...
    ///         } else {
    ///             // do else ...
    ///         }
    ///     },
    ///     _                       => { }
    /// }
    /// ```
    ///
    pub fn is_button_pressed(&self, res: ButtonResult) -> bool {
        match self.buttons.iter().find(|x| x.result() == res) {
            Some(i) => i.state(),
            _       => panic!("Not a valid button result for\
                               Dialog::is_button_checked()")
        }
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

impl HasSize for Dialog {
    fn size(&self) -> Size {
        self.frame.size()
    }
}
