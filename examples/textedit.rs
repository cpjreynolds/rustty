extern crate rustty;

use std::time::Duration;

use rustty::{Terminal, Event, Color};

struct Cursor {
    pos: Position,
    lpos: Position,
    color: Color,
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
        color: Color::Red,
    };
    let mut term = Terminal::new().unwrap();
    term[(cursor.pos.x, cursor.pos.y)].set_bg(cursor.color);
    term.swap_buffers().unwrap();
    loop {
        let evt = term.get_event(Some(Duration::from_millis(100))).unwrap();
        if let Some(Event::Char(ch)) = evt {
            match ch {
                '`' => {
                    break;
                }
                '\x7f' => {
                    cursor.lpos = cursor.pos;
                    if cursor.pos.x == 0 {
                        cursor.pos.y = cursor.pos.y.saturating_sub(1);
                    } else {
                        cursor.pos.x -= 1;
                    }
                    term[(cursor.pos.x, cursor.pos.y)].set_ch(' ');
                }
                '\r' => {
                    cursor.lpos = cursor.pos;
                    cursor.pos.x = 0;
                    cursor.pos.y += 1;
                }
                c @ _ => {
                    term[(cursor.pos.x, cursor.pos.y)].set_ch(c);
                    cursor.lpos = cursor.pos;
                    cursor.pos.x += 1;
                }
            }
            if cursor.pos.x >= term.cols() - 1 {
                term[(cursor.lpos.x, cursor.lpos.y)].set_bg(Color::Default);
                cursor.lpos = cursor.pos;
                cursor.pos.x = 0;
                cursor.pos.y += 1;
            }
            if cursor.pos.y >= term.rows() - 1 {
                term[(cursor.lpos.x, cursor.lpos.y)].set_bg(Color::Default);
                cursor.lpos = cursor.pos;
                cursor.pos.x = 0;
                cursor.pos.y = 0;
            }
            term[(cursor.lpos.x, cursor.lpos.y)].set_bg(Color::Default);
            term[(cursor.pos.x, cursor.pos.y)].set_bg(cursor.color);
            term.swap_buffers().unwrap();
        }
    }
}
