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
    Button
};

use rustty::ui::{
    Dialog,
    Label,
    StdButton,
    VerticalLayout,
    HorizontalLayout
};

fn boxify(vec: Vec<StdButton>) -> Vec<Box<Button>> {
    vec.into_iter().map(Box::new).map(|x| x as Box<Button>).collect()
}

fn create_maindlg() -> Dialog {
    let mut maindlg = Dialog::new(55, 12);

    // Text and alignment data to be used for displaying to dialog 
    let mut label1 = Label::new(45, 2);

    label1.set_text("Hello! This is a showcase of the ui module! \
                    Here's a horizontal layout configuration.");
    // Text is aligned in respect to the label, usually don't want margins
    label1.align_text(HorizontalAlign::Middle, VerticalAlign::Top, (0, 0));    
    label1.pack(&maindlg, HorizontalAlign::Middle, VerticalAlign::Top, (0, 1));
    maindlg.add_label(label1);

    let b1 = StdButton::new("Quit", 'q', ButtonResult::Ok);
    let b2 = StdButton::new("Foo!", 'f', ButtonResult::Custom(1));
    let b3 = StdButton::new("Bar!", 'a', ButtonResult::Custom(2));
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
    maindlg.draw_box();
    maindlg
}

fn create_vdlg(rows: usize) -> Dialog {
    let mut vdlg = Dialog::new(20, rows/4);

    // Text and alignment data to be used for displaying to dialog
    let mut label = Label::from_str("Vertical layout");
    label.pack(&vdlg, HorizontalAlign::Middle, VerticalAlign::Top, (0,1));
    vdlg.add_label(label);

    let b1 = StdButton::new("Yhh!", 'y', ButtonResult::Custom(1));
    let b2 = StdButton::new("Vpp!", 'v', ButtonResult::Custom(2));
    let b3 = StdButton::new("Wgg!", 'w', ButtonResult::Custom(3));
   
    let mut vlayout = VerticalLayout::from_vec(boxify(vec![b1, b2, b3]), 0);
    vlayout.pack(&vdlg, HorizontalAlign::Middle, VerticalAlign::Bottom, (0, 2));
    vdlg.add_layout(vlayout);

    vdlg.draw_box();
    vdlg
}


fn main() {
    let mut term = Terminal::new().unwrap();
    let mut maindlg = create_maindlg();
    let mut vdlg = create_vdlg(term.rows());
    // Align main dialog frame with the middle of the screen, and vdlg with the bottom
    maindlg.pack(&term, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));
    vdlg.pack(&term, HorizontalAlign::Left, VerticalAlign::Middle, (0,0));
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

                    let mut result = Label::from_str(msg);
                    result.pack(&maindlg, HorizontalAlign::Middle, VerticalAlign::Middle, (0,1));
                    result.draw(maindlg.frame_mut());
                },
                _ => {},
            }

            match vdlg.result_for_key(ch) {
                Some(ButtonResult::Custom(i)) => {
                    let msg = match i {
                        1 => "Yhh!",
                        2 => "Vpp!",
                        3 => "Wgg!",
                        _ => ""
                    };

                    let mut result = Label::from_str(msg);
                    result.pack(&vdlg, HorizontalAlign::Middle, VerticalAlign::Middle, (0,1));
                    result.draw(vdlg.frame_mut());
                },
                _ => {},
            }
        }

        // Draw widgets to screen
        maindlg.draw(&mut term);
        vdlg.draw(&mut term);
        term.swap_buffers().unwrap();
    }
}

