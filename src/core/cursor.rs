#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Cursor {
    Valid(usize, usize),
    Invalid,
}

