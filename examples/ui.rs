extern crate rustty;

use rustty::{
    Terminal,
    Event,
};

use rustty::ui::core::{
    Painter,
    Widget,
    ButtonResult,
    Alignable,
    HorizontalAlign,
    VerticalAlign,
    Button
};

use rustty::ui::{
    StdButton,
    Dialog,
    HorizontalLayout
};

fn create_maindlg() -> Dialog {
    let mut maindlg = Dialog::new(60, 10);
    maindlg.window_mut().draw_box();

    /*
    let mut b1 = StdButton::new("Quit", 'q', ButtonResult::Ok);
    b1.pack(&maindlg, HorizontalAlign::Left, VerticalAlign::Top, 6);
    let mut b2 = StdButton::new("Foo", 'f', ButtonResult::Custom(1));
    b2.pack(&maindlg, HorizontalAlign::Middle, VerticalAlign::Top, 6);
    let mut b3 = StdButton::new("Bar", 'b', ButtonResult::Custom(2));
    b3.pack(&maindlg, HorizontalAlign::Right, VerticalAlign::Top, 6);
    */

    let b1 = StdButton::new("Quit", 'q', ButtonResult::Ok);
    let b2 = StdButton::new("Foo", 'f', ButtonResult::Custom(1));
    let b3 = StdButton::new("Bar", 'b', ButtonResult::Custom(2));

    //let buttons = vec![b1, b2, b3].into_iter().map(Box::new).collect::<Vec<Box<Widget>>>();
    let mut hl1 = HorizontalLayout::from_vec(vec![b1, b2, b3].into_iter().map(Box::new).map(|x| x as Box<Button>).collect(), 1);
    hl1.pack(&maindlg, HorizontalAlign::Left, VerticalAlign::Top, (1,1));
    maindlg.add_layout(hl1);
    maindlg
}

fn main() {
    let mut term = Terminal::new().unwrap();
    let mut maindlg = create_maindlg();
    maindlg.window_mut().align(&term, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match maindlg.result_for_key(ch) {
                Some(ButtonResult::Ok)          => break 'main,
                Some(ButtonResult::Custom(i))   => {
                    let msg = if i == 1 { "Foo!" } else { "Bar!" };
                    let w = maindlg.window_mut();
                    let x = w.halign_line(msg, HorizontalAlign::Middle, 1);
                    let y = w.valign_line(msg, VerticalAlign::Middle, 1);
                    w.printline(x, y, msg);
                },
                _ => {},
            }
        }

        maindlg.window().draw_into(&mut term);
        term.swap_buffers().unwrap();
    }
}

