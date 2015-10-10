extern crate rustty;

use rustty::{
    Terminal,
    Event,
    HasSize,
};

const BLOCK: char = '\u{25AA}';
const BUFF_ALIGN: usize = 35;

fn write_word(term: &mut Terminal, row: usize, word: String) { 
    // Helper function to write right-aligned text to the passed
    // row, aligns via the BUFF_ALIGN constant
    let cols = term.cols() as usize;

    for (idx, ch) in word.chars().enumerate() { 
        let x = cols - BUFF_ALIGN;
        term[(x + idx, row)].set_ch(ch);
    }
}

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

        // Grab the size of the console, and reserve 4 rows at the bottom
        // to write text to. These bottom 4 will NOT be rendered
        let (cols, rows) = term.size();
        let (cols, rows) = (cols as isize, (rows - 4) as isize);

        let (a, b) = (cols / 2, rows / 2);

        // Main render loop, draws the circle
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

        // Render text at bottom
        let i = cols*rows;
        let y = i as isize / cols;
        write_word(&mut term, y as usize, "+ -> Increase Radius".to_string());
        write_word(&mut term, (y+1) as usize, "- -> Decrease Radius".to_string());
        write_word(&mut term, (y+2) as usize, "q -> Quit".to_string());
        
        // Swap buffers
        term.swap_buffers().unwrap();
    }
}
