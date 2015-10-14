extern crate rustty;

use rustty::{
    Terminal,
    Event,
};

use rustty::ui::{
    Painter,
    Dialog,
    DialogResult,
    Alignable,
    HorizontalAlign,
    VerticalAlign,
    ButtonLayout
};

fn create_maindlg() -> Dialog {
    // Create a dialog window 60 units wide and 12 units high
    let mut maindlg = Dialog::new(60, 12);

    // Text and alignment data to be used for displaying to dialog
    let s = "Hello! This is a showcase of the ui module!";
    let s2 = "Here's a vertical layout configuration.";
    let x = maindlg.window().halign_line(s, HorizontalAlign::Middle, 1);
    let x2 = maindlg.window().halign_line(s2, HorizontalAlign::Middle, 0);
    maindlg.window_mut().printline(x, 2, s);
    maindlg.window_mut().printline(x2, 3, s2);
    
    // Create the left column of buttons, these are buttons 0, 1, 2
    maindlg.add_button("Foo", 'f', DialogResult::Custom(1));
    maindlg.add_button("Bar", 'b', DialogResult::Custom(2));
    maindlg.add_button("Quit", 'q', DialogResult::Ok);
    maindlg.draw_buttons_subset(0, 3, ButtonLayout::Vertical(HorizontalAlign::Left));

    // Create the middle column of buttons, these are buttons 3, 4, 5
    maindlg.add_button("Juu", 'j', DialogResult::Custom(3));
    maindlg.add_button("Too", 't', DialogResult::Custom(4));
    maindlg.add_button("Boo", 'b', DialogResult::Custom(5));
    maindlg.draw_buttons_subset(3, 6, ButtonLayout::Vertical(HorizontalAlign::Middle));

    // Create the right column of buttons, these are buttons 6, 7, 8
    maindlg.add_button("Yhh", 'y', DialogResult::Custom(6));
    maindlg.add_button("Vpp", 'v', DialogResult::Custom(7));
    maindlg.add_button("Wgg", 'w', DialogResult::Custom(8));
    maindlg.draw_buttons_subset(6, 9, ButtonLayout::Vertical(HorizontalAlign::Right));

    // Draw the outline for the dialog
    maindlg.window_mut().draw_box();
    maindlg
}

fn create_hdlg(cols: usize) -> Dialog {
    // Create a dialog window that's as wide as the terminal, and 7 units high
    let mut hdlg = Dialog::new(cols, 7);

    // Text and alignment data to be used for displaying to dialog
    let s = "Here's a horizontal layout configuration.";
    let x = hdlg.window().halign_line(s, HorizontalAlign::Middle, 1);
    hdlg.window_mut().printline(x, 2, s);

    // Create a row of buttons to be displayed
    hdlg.add_button("Lo", 'l', DialogResult::Ok);
    hdlg.add_button("Yo", 'y', DialogResult::Ok);
    hdlg.add_button("Ho", 'h', DialogResult::Ok);
    hdlg.add_button("Uo", 'u', DialogResult::Ok);

    // Draw all buttons horizontally, and draw outline of dialog
    hdlg.draw_buttons(ButtonLayout::Horizontal(2));
    hdlg.window_mut().draw_box();
    hdlg
}


fn main() {
    let mut term = Terminal::new().unwrap();
    let mut maindlg = create_maindlg();
    let mut hdlg = create_hdlg(term.cols());
    // Align main dialog window with the middle of the screen, and hdlg with the bottom
    maindlg.window_mut().align(&term, HorizontalAlign::Middle, VerticalAlign::Middle, 0);
    hdlg.window_mut().align(&term, HorizontalAlign::Middle, VerticalAlign::Bottom, 0);
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match maindlg.result_for_key(ch) {
                Some(DialogResult::Ok) => break 'main,
                Some(DialogResult::Custom(i)) => {
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
        }

        // Draw widgets to screen
        maindlg.window().draw_into(&mut term);
        hdlg.window().draw_into(&mut term);
        term.swap_buffers().unwrap();
    }
}

