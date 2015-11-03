extern crate rustty;

use rustty::{Terminal, Event, HasSize};
use rustty::ui::core::{Widget, ButtonResult, HorizontalAlign, VerticalAlign};
use rustty::ui::{StdButton, Dialog, Label};

fn create_dlg() -> Dialog {
    let mut maindlg = Dialog::new(80, 15);
    maindlg.draw_box();

    let mut b1 = StdButton::new("Quit", 'q', ButtonResult::Ok);
    b1.pack(&maindlg, HorizontalAlign::Middle, VerticalAlign::Bottom, (0, 3));

    maindlg.add_button(b1);

    let mut label = Label::new(10, 4);
    //label.set_text("Yes this is a lot to dislpay");
    label.set_text(  "Hi yes ok i would like toaaa");
    //assert_eq!(true, false);
    label.align_text(HorizontalAlign::Left, VerticalAlign::Top, (1,0));
    label.pack(&maindlg, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));
    label.draw_box();

    maindlg.add_label(label);

    maindlg
}

fn main() {
    let mut term = Terminal::new().unwrap();
    let mut maindlg = create_dlg();
    maindlg.pack(&term, HorizontalAlign::Middle, VerticalAlign::Middle, (0, 0));
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match maindlg.result_for_key(ch) {
                Some(ButtonResult::Ok)  => break 'main,
                _                       => {}
            }
        }

        maindlg.draw(&mut term);
        term.swap_buffers().unwrap();
    }
}