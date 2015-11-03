use std::ascii::AsciiExt;
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

/// Display text to widgets
///
/// # Examples
///
/// ```
/// use rustty::ui::core::{VerticalAlign, HorizontalAlign, Widget};
/// use rustty::ui::{Dialog, Label};
///
/// let mut maindlg = Dialog::new(60, 10);
///
/// let mut label = Label::from_str("Hi, this is an example!");
/// label.pack(&maindlg, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));
///
/// maindlg.add_label(label);
/// maindlg.draw_box();
/// ```
///
pub struct Label {
    frame: Frame,
    text: String,
    x: usize,
    y: usize,
    t_halign: HorizontalAlign,
    t_valign: VerticalAlign,
    t_margin: (usize, usize),
}

impl Label {
    /// Construct a new Label widget `cols` wide by `rols` high. Initial text is empty
    /// and left aligned
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::ui::Label;
    ///
    /// let mut label = Label::new(60, 10);
    /// ```
    ///
    pub fn new(cols: usize, rows: usize) -> Label {
        Label {
            frame: Frame::new(cols, rows),
            text: "".to_string(),
            x: 0,
            y: 0,
            t_halign: HorizontalAlign::Left,
            t_valign: VerticalAlign::Middle,
            t_margin: (0, 0)
        }
    }

    /// Construct a new label widget from an existing string *s*. *s* can either be a
    /// `&str` or `String` , and a label will be constructed that is the size of the 
    /// length of characters in *s*. Text is left aligned by default
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::ui::Label;
    ///
    /// let mut label1 = Label::from_str("This is a label");    // label is size (15x1)
    ///
    /// let s = "Here's another label".to_string();
    /// let mut label2 = Label::from_str(s);                    // label is size (20x1)
    /// ```
    ///
    pub fn from_str<S: Into<String>>(s: S) -> Label {
        let s = s.into();
        Label {
            frame: Frame::new(s.len(), 1),
            text: s.into(),
            x: 0,
            y: 0,
            t_halign: HorizontalAlign::Left,
            t_valign: VerticalAlign::Middle,
            t_margin: (1, 1)
        }
    }

    /// Specify a custom alignment for the text within the widget
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::ui::core::{HorizontalAlign, VerticalAlign};
    /// use rustty::ui::Label;
    ///
    /// let mut label = Label::new(20, 3);
    /// label.set_text("Centered");
    /// label.align_text(HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));
    /// ```
    ///
    pub fn align_text(&mut self, halign: HorizontalAlign, valign: VerticalAlign,
                    margin: (usize, usize)) {
        self.t_halign = halign.clone();
        self.t_valign = valign.clone();
        self.t_margin = margin;

        self.x = self.frame.halign_line(&self.text, halign, margin.0);
        self.y = self.frame.valign_line(&self.text, valign, margin.1);
    }

    /// Set the text of the widget to the passed `&str` or `String`. If the
    /// widget does not have enough room to display the new text, the widget 
    /// is resized **horizontally** to accomodate the new size
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::HasSize;
    /// use rustty::ui::core::Widget;
    /// use rustty::ui::Label;
    ///
    /// let mut label1 = Label::new(20, 3);
    /// label1.set_text("Initial text");
    ///
    /// let mut label2 = Label::from_str("too small");  // label is size (9x1)
    /// label2.set_text("This is too big");             // label is size (15x1)
    /// assert_eq!(label2.frame().size(), (15, 1));
    ///
    /// let mut label3 = Label::new(4, 2);              // label is size (4x2)
    /// label3.set_text("Too big to fit!");             // label is size (8x2) 
    /// assert_eq!(label3.frame().size(), (8, 2));
    /// ```
    ///
    pub fn set_text<S: Into<String>>(&mut self, new_str: S) {
        
        let new_str = new_str.into();
        self.text = new_str; 
        let (framex, framey) = self.frame.size();
        if self.text.len() > (framex * framey) {
            // Extend widget horizontally such that it can accomodate the new
            // text size, algorithm to determine new horizontal size is:
            //      L   : length of new text
            //      CxR : dimensions of widget 
            //      ciel(L - C * R) / R + C
            let new_x = (self.text.len() - (framex-1) * framey) as f32 / framey as f32;
            let new_x = new_x.ceil() as usize + framex;
            self.frame.resize((new_x, framey));
        }
    }
}

impl Widget for Label {
    fn draw(&mut self, parent: &mut CellAccessor) {
        let mut split_parts: Vec<String> = vec!["".to_string()];
        let mut parse = self.text.clone();
        let frame_width = self.frame.size().0;
        // This loop below will accomplish splitting a line of text
        // into lines that adhere to the amount of rows in a label
        loop {
            // Look for a word until a whitespace is reached
            if let Some(loc) = parse.find(char::is_whitespace) {
                let line_len = split_parts.last().unwrap().len();
                let tmp = parse[..loc].to_owned();
                // If the word can fit on the current line, add it
                if line_len + tmp.len() + self.t_margin.0 < frame_width {
                    split_parts.last_mut().unwrap().push_str(&tmp);
                } else {
                    split_parts.push(tmp.to_owned());
                }
                parse = parse[loc..].to_owned();
            } else {
                // If no whitespace detected, there may still be one
                // more word so attempt to add it
                if parse.len() != 0 {
                    let line_len = split_parts.last().unwrap().len();
                    if line_len + parse.len() + self.t_margin.0 < frame_width {
                        split_parts.last_mut().unwrap().push_str(&parse);
                    } else {
                        split_parts.push(parse);
                    }
                }
                break;
            }

            // Look for the range of spaces between words
            if let Some(loc) = parse.find(|c: char| c.is_ascii() && c != ' ') {
                let line_len = split_parts.last().unwrap().len();
                let tmp = parse[..loc].to_owned();
                // If the next word can fit on the current line, do so
                if line_len + tmp.len() + self.t_margin.0 < frame_width {
                    split_parts.last_mut().unwrap().push_str(&tmp);
                } else {
                    split_parts.push("".to_string());
                }
                parse = parse[loc..].to_owned();
            } else {
                // We don't care if there's spaces at the end, so don't check
                break;
            }
        }
      
        if split_parts.len() > self.frame.size().1 - self.t_margin.1 {
            self.frame.resize((frame_width, split_parts.len()));
        }

        for (i, item) in split_parts.iter().enumerate() {
            self.frame.printline(self.x, self.y + i, &item);
        }
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
