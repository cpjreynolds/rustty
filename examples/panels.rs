extern crate rustty;

use std::io::Result;

use rustty::{Terminal, Event, Draw, Border, Panel, Cell, Color, Attr};

fn main() {
    match execute() {
        Ok(..) => {}
        Err(e) => println!("encountered error: {}", e),
    }
}

fn execute() -> Result<()> {
    let mut term = try!(Terminal::new());

    let border = Border::default();

    border.draw(0, 0, term.panel_mut());

    let mut subx = 0;
    let mut suby = 0;
    let mut subpanel = Panel::with_size(10, 5, Cell::default());
    let blankpanel = Panel::with_size(10, 5, Cell::default());
    border.draw(0, 0, &mut subpanel);

    subpanel.draw(subx, suby, term.panel_mut());

    term.refresh();

    'main: loop {
        let cols = term.cols();
        let rows = term.rows();

        blankpanel.draw(subx, suby, term.panel_mut());
        border.draw(0, 0, term.panel_mut());

        for evt in try!(term.poll_events()) {
            if let Event::Key(ch) = evt {
                match ch {
                    'q' => break 'main,
                    'j' => suby = suby.wrapping_add(1) % rows,
                    'k' => suby = suby.wrapping_sub(1) % rows,
                    'l' => subx = subx.wrapping_add(1) % cols,
                    'h' => subx = subx.wrapping_sub(1) % cols,
                    _ => {}
                }
            }
        }

        subpanel.draw(subx, suby, term.panel_mut());

        term.refresh();
    }

    Ok(())
}
