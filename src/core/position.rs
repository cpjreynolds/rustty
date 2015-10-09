/// A `(x, y)` position on screen.
pub type Pos = (usize, usize);
/// A `(cols, rows)` size.
pub type Size = (usize, usize);

pub trait HasSize {
    fn size(&self) -> Size;
}

pub trait HasPosition {
    fn origin(&self) -> Pos;
    fn set_origin(&mut self, new_origin: Pos);
}
