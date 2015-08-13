extern crate rustty;

use rustty::{
    Terminal,
    Event,
};

const BLOCK: char = '\u{25AA}';

fn main() {
    let mut term = Terminal::new().unwrap();
    let mut radius = 10isize;
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match ch {
                'q' => break 'main,
                '+' => radius += 1,
                '-' => radius -= 1,
                _ => {},
            }
        }

        let (cols, rows) = term.size();
        let (cols, rows) = (cols as isize, rows as isize);

        let (a, b) = (cols / 2, rows / 2);

        for (i, cell) in term.iter_mut().enumerate() {
            let y = i as isize / cols;
            let x = i as isize % cols;

            if ((x - a).pow(2) + (y - b).pow(2)) <= radius.pow(2) {
                cell.set_ch(BLOCK);
            } else {
                cell.set_ch(' ');
            }
        }
        term.swap_buffers().unwrap();
    }
}
