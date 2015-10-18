extern crate rustty;

use rustty::{
    Terminal,
    Event,
};
use rustty::ui::{
    Painter,
    Dialog,
    ButtonResult,
    Alignable,
    HorizontalAlign,
    VerticalAlign,
    Button,
    StdButton
};

fn create_maindlg() -> Dialog {
    let mut maindlg = Dialog::new(60, 10);
    /*
    let s = "Hello! This is a showcase of the ui module!";
    let x = maindlg.window().halign_line(s, HorizontalAlign::Middle, 1);
    maindlg.window_mut().printline(x, 2, s);
    
    maindlg.add_button("Foo", 'f', DialogResult::Custom(1));
    maindlg.add_button("Bar", 'b', DialogResult::Custom(2));
    maindlg.add_button("Quit", 'q', DialogResult::Ok);
    maindlg.draw_buttons(ButtonLayout::Horizontal(2));
    */    
    maindlg.window_mut().draw_box();
    let mut b1 = StdButton::new("Foo", 'f', ButtonResult::Custom(1));
    b1.align(&maindlg, HorizontalAlign::Left, VerticalAlign::Middle, 0);
    maindlg.add_button(b1);
    maindlg
}

fn main() {
    let mut term = Terminal::new().unwrap();
    let mut maindlg = create_maindlg();
    maindlg.window_mut().align(&term, HorizontalAlign::Middle, VerticalAlign::Middle, 0);
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match maindlg.result_for_key(ch) {
                Some(ButtonResult::Ok) => break 'main,
                Some(ButtonResult::Custom(i)) => {
                    /*
                    let msg = if i == 1 { "Foo!" } else { "Bar!" };
                    let w = maindlg.window_mut();
                    let x = w.halign_line(msg, HorizontalAlign::Middle, 1);
                    let y = w.valign_line(msg, VerticalAlign::Middle, 1);
                    w.printline(x, y, msg);
                    */
                },
                _ => {},
            }
        }

        maindlg.window().draw_into(&mut term);
        term.swap_buffers().unwrap();
    }
}

