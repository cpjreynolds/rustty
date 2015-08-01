extern crate rustty;

use rustty::{
    Terminal,
    Event,
    Style,
    Color,
};

struct Cursor {
    pos: Position,
    lpos: Position,
    style: Style,
}

#[derive(Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

fn main() {
    let mut cursor = Cursor {
        pos: Position { x: 0, y: 0 },
        lpos: Position { x: 0, y: 0 },
        style: Style::with_color(Color::Red),
    };
    let mut term = Terminal::new().unwrap();
    term[(cursor.pos.x, cursor.pos.y)].set_bg(cursor.style);
    term.swap_buffers().unwrap();
    loop {
        let evt = term.get_event(100).unwrap();
        if let Some(Event::Key(ch)) = evt {
            match ch {
                '`' => {
                    break;
                },
                '\x7f' => {
                    cursor.lpos = cursor.pos;
                    if cursor.pos.x == 0 {
                        cursor.pos.y = cursor.pos.y.saturating_sub(1);
                    } else {
                        cursor.pos.x -= 1;
                    }
                    term[(cursor.pos.x, cursor.pos.y)].set_ch(' ');
                },
                '\r' => {
                    cursor.lpos = cursor.pos;
                    cursor.pos.x = 0;
                    cursor.pos.y += 1;
                },
                c @ _ => {
                    term[(cursor.pos.x, cursor.pos.y)].set_ch(c);
                    cursor.lpos = cursor.pos;
                    cursor.pos.x += 1;
                },
            }
            if cursor.pos.x >= term.cols()-1 {
                term[(cursor.lpos.x, cursor.lpos.y)].set_bg(Style::default());
                cursor.lpos = cursor.pos;
                cursor.pos.x = 0;
                cursor.pos.y += 1;
            }
            if cursor.pos.y >= term.rows()-1 {
                term[(cursor.lpos.x, cursor.lpos.y)].set_bg(Style::default());
                cursor.lpos = cursor.pos;
                cursor.pos.x = 0;
                cursor.pos.y = 0;
            }
            term[(cursor.lpos.x, cursor.lpos.y)].set_bg(Style::default());
            term[(cursor.pos.x, cursor.pos.y)].set_bg(cursor.style);
            term.swap_buffers().unwrap();
        }
    }
}

