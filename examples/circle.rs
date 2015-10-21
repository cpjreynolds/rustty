extern crate rustty;

use rustty::{
    Terminal,
    Event,
    HasSize,
    CellAccessor
};

use rustty::ui::core::{
    Painter,
    Widget,
    Base,
    Alignable,
    HorizontalAlign,
    VerticalAlign,
    ButtonResult
};

use rustty::ui::{
    Dialog,
    StdButton
};

const BLOCK: char = '\u{25AA}';

fn create_optiondlg() -> Dialog {
    let mut optiondlg = Dialog::new(50, 6);
    let mut inc_b = StdButton::new(" + :Increase Radius", '+', ButtonResult::Custom(1));
    inc_b.pack(&optiondlg, HorizontalAlign::Left, VerticalAlign::Top, 1);
    let mut dec_b = StdButton::new(" - :Decrease Radius", '-', ButtonResult::Custom(2));
    dec_b.pack(&optiondlg, HorizontalAlign::Left, VerticalAlign::Middle, 1);
    let mut quit_b = StdButton::new("Quit", 'q', ButtonResult::Ok);
    quit_b.pack(&optiondlg, HorizontalAlign::Left, VerticalAlign::Bottom, 1);
    
    optiondlg.add_button(inc_b);
    optiondlg.add_button(dec_b);
    optiondlg.add_button(quit_b);

    optiondlg.window_mut().draw_box();
    optiondlg
}

fn main() {
    // Create our terminal, dialog window and main canvas
    let mut term = Terminal::new().unwrap();
    let mut optiondlg = create_optiondlg();
    let mut canvas = Base::new(term.size().0, term.size().1 - 4);

    // Align canvas to top left, and dialog to bottom right
    optiondlg.window_mut().align(&term, HorizontalAlign::Right, VerticalAlign::Bottom, 0);
    canvas.align(&term, HorizontalAlign::Left, VerticalAlign::Top, 0);
    
    let mut radius = 10u32;
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match optiondlg.result_for_key(ch) {
                Some(ButtonResult::Ok)          => break 'main,
                Some(ButtonResult::Custom(i))   => {
                    radius = 
                        if i == 1 { 
                            radius.saturating_add(1) 
                        } else {
                            radius.saturating_sub(1)
                        };
                },
                _ => {},
            }
        }
        // Grab the size of the canvas
        let (cols, rows) = canvas.size();
        let (cols, rows) = (cols as isize, rows as isize);

        let (a, b) = (cols / 2, rows / 2);

        // Main render loop, draws the circle to canvas
        for i in 0..cols*rows {
            let y = i as isize / cols;
            let x = i as isize % cols;

            let mut cell = canvas.get_mut(x as usize, y as usize).unwrap();

            if ((x - a).pow(2)/4 + (y - b).pow(2)) <= radius.pow(2) as isize {
                cell.set_ch(BLOCK);
            } else {
                cell.set_ch(' ');
            }
        }

        // draw the canvas, dialog window and swap buffers
        canvas.draw_into(&mut term);
        optiondlg.window().draw_into(&mut term);
        term.swap_buffers().unwrap();
    }
}
