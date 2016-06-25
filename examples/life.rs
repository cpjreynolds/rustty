extern crate rustty;
extern crate rand;

use std::io::Error;
use std::thread;
use std::time::Duration;
use std::num::Wrapping;

use rand::Rng;

use rustty::{Terminal, Color, Event};

const BLOCK: char = '\u{25AA}';

const BOOL_WEIGHT: u32 = 5;

fn main() {
    match execute() {
        Ok(..) => {}
        Err(e) => {
            println!("encountered error: {}", e);
        }
    }
}

fn execute() -> Result<(), Error> {
    let mut terminal = try!(Terminal::new());
    let mut rng = rand::thread_rng();

    for cell in &mut *terminal {
        if rng.gen_weighted_bool(BOOL_WEIGHT) {
            cell.set_ch('x');
        }
    }


    try!(terminal.refresh());

    'main: loop {
        while let Some(Event::Key(ch)) = try!(terminal.get_event(None)) {
            match ch {
                'q' => {
                    break 'main;
                }
                ' ' => {
                    break;
                }
                _ => {}
            }
        }

        let mut sums = Vec::new();

        for y in 0..terminal.rows() {
            for x in 0..terminal.cols() {
                let mut sum = 0;

                for (nx, ny) in idxs(terminal.rows(), terminal.cols(), x, y) {
                    if terminal[(nx, ny)].ch() == 'x' {
                        sum += 1;
                    }
                }

                sums.push(sum);
            }
        }

        for (i, cell) in terminal.iter_mut().enumerate() {
            let sum = sums[i];
            if sum == 3 {
                cell.set_ch('x');
            } else if sum != 4 {
                cell.set_ch(' ');
            }
        }

        try!(terminal.refresh());
    }

    Ok(())
}

fn idxs(rows: usize, cols: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut buf = Vec::new();

    let xs1 = x.wrapping_sub(1) % cols;
    let xp1 = x.wrapping_add(1) % cols;

    let ys1 = y.wrapping_sub(1) % rows;
    let yp1 = y.wrapping_add(1) % rows;

    buf.push((xs1, ys1));
    buf.push((x, ys1));
    buf.push((xp1, ys1));
    buf.push((xs1, y));
    buf.push((x, y));
    buf.push((xp1, y));
    buf.push((xs1, yp1));
    buf.push((x, yp1));
    buf.push((xp1, yp1));

    buf
}
