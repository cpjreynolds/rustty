#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Cursor {
    Valid(usize, usize),
    Invalid,
}

impl Cursor {
    pub fn is_next(&self, cursor: Cursor) -> bool {
        if let Cursor::Valid(cx, cy) = cursor {
            if let &Cursor::Valid(sx, sy) = self {
                (cx, cy) != (sx + 1, sy)
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

