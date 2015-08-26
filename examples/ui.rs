extern crate rustty;

use rustty::{
    Terminal,
    Event,
};
use rustty::ui::{
    Widget,
    Painter,
    Alignable,
    HorizontalAlign,
    VerticalAlign,
    create_button,
};

fn create_maindlg() -> Widget {
    let mut maindlg = Widget::new(60, 10);
    let s = "Hello! This is a showcase of the ui module!";
    let x = maindlg.halign_line(s, HorizontalAlign::Middle, 1);
    maindlg.printline(x, 2, s);
    let mut b = create_button("Quit", Some('q'));
    b.align(&maindlg, HorizontalAlign::Middle, VerticalAlign::Bottom, 1);
    b.draw_into(&mut maindlg);
    maindlg.draw_box();
    maindlg
}

fn main() {
    let mut term = Terminal::new().unwrap();
    let mut maindlg = create_maindlg();
    maindlg.align(&term, HorizontalAlign::Middle, VerticalAlign::Middle, 0);
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match ch {
                'q' | 'Q' => break 'main,
                _ => {},
            }
        }

        maindlg.draw_into(&mut term);
        term.swap_buffers().unwrap();
    }
}

