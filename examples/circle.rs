extern crate rustty;

use rustty::{
    Terminal,
    Event,
};

const BLOCK: char = '\u{25AA}';

fn main() {
    let mut term = Terminal::new().unwrap();
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

        let (cols, rows) = term.size();
        let (cols, rows) = (cols as isize, (rows - 3) as isize);

        let (a, b) = (cols / 2, rows / 2);

        for i in 0..cols*rows {
            let y = i as isize / cols;
            let x = i as isize % cols;

            let mut cell = &mut term[(x as usize, y as usize)];

            if ((x - a).pow(2)/4 + (y - b).pow(2)) <= radius.pow(2) as isize {
                cell.set_ch(BLOCK);
            } else {
                cell.set_ch(' ');
            }
        }
        term.swap_buffers().unwrap();
    }
}
