extern crate rustty;
extern crate rand;

use std::thread;

use rustty::{
    Terminal,
    Event,
};
use rand::Rng;

const BLOCK: char = '\u{2588}';

enum Mode {
    Chars,
    Blocks,
}

impl Mode {
    fn switch(&mut self) {
        match *self {
            Mode::Chars => *self = Mode::Blocks,
            Mode::Blocks => *self = Mode::Chars,
        }
    }
}

fn main() {
    let mut term = Terminal::new().unwrap();
    let mut rng = rand::thread_rng();
    let mut mode = Mode::Blocks;
    let mut rate = 100;
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match ch {
                'q' => break 'main,
                ' ' => mode.switch(),
                '+' => {
                    if rate > 100 {
                        rate -= 100;
                    }
                },
                '-' => {
                    if rate < 1000 {
                        rate += 100;
                    }
                },
                _ => {},
            }
        }

        match mode {
            Mode::Chars => {
                for (cell, rch) in term.iter_mut().zip(rng.gen_ascii_chars()) {
                    cell.set_ch(rch);
                }
            },
            Mode::Blocks => {
                for (cell, set) in term.iter_mut().zip(rng.gen_iter::<bool>()) {
                    if set {
                        cell.set_ch(BLOCK);
                    } else {
                        cell.set_ch(' ');
                    }
                }
            },
        }
        term.swap_buffers().unwrap();
        thread::sleep_ms(rate);
    }
}
