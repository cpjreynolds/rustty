extern crate rustty;

use rustty::{
    Terminal,
    Event,
};

use rustty::ui::core::{
    Widget,
    HorizontalAlign,
    VerticalAlign,
    ButtonResult,
    Button,
    Frame
};

use rustty::ui::{
    Dialog,
    Label,
    StdButton,
    VerticalLayout,
    HorizontalLayout
}; 

struct RedButton {
    frame: Frame,
    accel: char,
    result: ButtonResult,
    text: String
}

impl RedButton {
    fn new(text: &str, accel: char, result: ButtonResult) -> RedButton {
        let s = format!("[ {} ]", text);
        let width = s.chars.count();
        let mut button =
            RedButton {
                frame: Frame::new(width, 1),
                accel: accel.to_lowercase().next().unwrap_or(accel),
                result: result,
                text: s 
            };
        button.frame.printline(0, 0, &button.text[..]);
        match find_accel_char_index(text, button.accel) {
            Some(i) => { 
                button.frame.get_mut(i+2, 0).unwrap().set_attrs(Attr::Bold);
            },
            None    => (),
        }
        button
    }
}
