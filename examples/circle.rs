extern crate rustty;

use rustty::{
    Terminal,
    Event,
    HasSize,
    CellAccessor
};

use rustty::ui::{
    Painter,
    Dialog,
    Widget,
    Alignable,
    HorizontalAlign,
    VerticalAlign
};

const BLOCK: char = '\u{25AA}';

fn create_optiondlg() -> Dialog {
    let mut optiondlg = Dialog::new(50, 6);
    let inc_label = "+ -> Increase Radius";
    let dec_label = "- -> Decrease Radius";
    let q_label = "q -> Quit";
    let inc_pos = optiondlg.window().halign_line(inc_label, HorizontalAlign::Left, 1);
    let dec_pos = optiondlg.window().halign_line(dec_label, HorizontalAlign::Left, 1);
    let q_pos = optiondlg.window().halign_line(q_label, HorizontalAlign::Left, 1);
    optiondlg.window_mut().printline(inc_pos, 1, inc_label);
    optiondlg.window_mut().printline(dec_pos, 2, dec_label);
    optiondlg.window_mut().printline(q_pos, 3, q_label);
    optiondlg.window_mut().draw_box();
    optiondlg
}

fn main() {
    // Create our terminal, dialog window and main canvas
    let mut term = Terminal::new().unwrap();
    let mut optiondlg = create_optiondlg();
    let mut canvas = Widget::new(term.size().0, term.size().1 - 4);

    // Align canvas to top left, and dialog to bottom right
    optiondlg.window_mut().align(&term, HorizontalAlign::Right, VerticalAlign::Bottom, 0);
    canvas.align(&term, HorizontalAlign::Left, VerticalAlign::Top, 0);
    
    let mut radius = 10u32;
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match ch {
                'q' => break 'main,
                '+' => radius = radius.saturating_add(1),
                '-' => radius = radius.saturating_sub(1),
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
