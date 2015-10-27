extern crate rustty;

use rustty::{
    Terminal,
    Event,
};

use rustty::ui::core::{
    Painter,
    Widget,
    HorizontalAlign,
    VerticalAlign,
    ButtonResult,
    Button
};

use rustty::ui::{
    Dialog,
    StdButton,
    VerticalLayout,
    HorizontalLayout
};

fn boxify(vec: Vec<StdButton>) -> Vec<Box<Button>> {
    vec.into_iter().map(Box::new).map(|x| x as Box<Button>).collect()
}

fn create_maindlg() -> Dialog {
    let mut maindlg = Dialog::new(60, 12);

    // Text and alignment data to be used for displaying to dialog
    let s = "Hello! This is a showcase of the ui module!";
    let s2 = "Here's a horizontal layout configuration.";
    let x = maindlg.window().halign_line(s, HorizontalAlign::Middle, 1);
    let x2 = maindlg.window().halign_line(s2, HorizontalAlign::Middle, 0);
    maindlg.window_mut().printline(x, 2, s);
    maindlg.window_mut().printline(x2, 3, s2);

    let b1 = StdButton::new("Quit", 'q', ButtonResult::Ok);
    let b2 = StdButton::new("Foo!", 'f', ButtonResult::Custom(1));
    let b3 = StdButton::new("Bar!", 'b', ButtonResult::Custom(2));
    let b4 = StdButton::new("Juu!", 'j', ButtonResult::Custom(3));
    let b5 = StdButton::new("Tuu!", 't', ButtonResult::Custom(4));
    let b6 = StdButton::new("Boo!", 'b', ButtonResult::Custom(5));

    let mut hlayout1 = HorizontalLayout::from_vec(boxify(vec![b1, b2, b3]), 1);
    hlayout1.pack(&maindlg, HorizontalAlign::Middle, VerticalAlign::Bottom, (0, 2));
    maindlg.add_layout(hlayout1);

    let mut hlayout2 = HorizontalLayout::from_vec(boxify(vec![b4, b5, b6]), 1);
    hlayout2.pack(&maindlg, HorizontalAlign::Middle, VerticalAlign::Bottom, (0, 3));
    maindlg.add_layout(hlayout2);

    // Draw the outline for the dialog
    maindlg.window_mut().draw_box();
    maindlg
}

fn create_hdlg(rows: usize) -> Dialog {
    let mut hdlg = Dialog::new(20, rows/4);

    // Text and alignment data to be used for displaying to dialog
    let s = "Vertical layout";
    let x = hdlg.window().halign_line(s, HorizontalAlign::Middle, 1);
    hdlg.window_mut().printline(x, 2, s);

    let b1 = StdButton::new("Yhh!", 'y', ButtonResult::Custom(1));
    let b2 = StdButton::new("Vpp!", 'v', ButtonResult::Custom(2));
    let b3 = StdButton::new("Wgg!", 'w', ButtonResult::Custom(3));
   
    let mut vlayout = VerticalLayout::from_vec(boxify(vec![b1, b2, b3]), 0);
    vlayout.pack(&hdlg, HorizontalAlign::Middle, VerticalAlign::Bottom, (0, 2));
    hdlg.add_layout(vlayout);

    hdlg.window_mut().draw_box();
    hdlg
}


fn main() {
    let mut term = Terminal::new().unwrap();
    let mut maindlg = create_maindlg();
    let mut hdlg = create_hdlg(term.rows());
    // Align main dialog window with the middle of the screen, and hdlg with the bottom
    maindlg.pack(&term, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));
    hdlg.pack(&term, HorizontalAlign::Left, VerticalAlign::Middle, (0,0));
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match maindlg.result_for_key(ch) {
                Some(ButtonResult::Ok) => break 'main,
                Some(ButtonResult::Custom(i)) => {
                    let msg = match i { 
                        1   =>  "Foo!", 
                        2   =>  "Bar!",
                        3   =>  "Juu!",
                        4   =>  "Too!",
                        5   =>  "Boo!",
                        6   =>  "Yhh!",
                        7   =>  "Vpp!",
                        8   =>  "Wgg!",
                        _   =>  ""
                    };
                    let w = maindlg.window_mut();
                    let x = w.halign_line(msg, HorizontalAlign::Middle, 1);
                    let y = w.valign_line(msg, VerticalAlign::Middle, 1);
                    w.printline(x, y, msg);
                },
                _ => {},
            }

            match hdlg.result_for_key(ch) {
                Some(ButtonResult::Custom(i)) => {
                    let msg = match i {
                        1 => "Yhh!",
                        2 => "Vpp!",
                        3 => "Wgg!",
                        _ => ""
                    };
                    let w = hdlg.window_mut();
                    let x = w.halign_line(msg, HorizontalAlign::Middle, 1);
                    let y = w.valign_line(msg, VerticalAlign::Middle, 1);
                    w.printline(x, y, msg);
                },
                _ => {},
            }
        }

        // Draw widgets to screen
        maindlg.window().draw_into(&mut term);
        hdlg.window().draw_into(&mut term);
        term.swap_buffers().unwrap();
    }
}

