#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Cursor {
    Valid(usize, usize),
    Invalid,
}

impl Cursor {
    pub fn next(&self) -> Cursor {
        if let Cursor::Valid(x, y) = *self {
            Cursor::Valid(x+1, y)
        } else {
            Cursor::Invalid
        }
    }
}
