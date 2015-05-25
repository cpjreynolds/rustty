
/// A coordinate that can be either valid or invalid.
#[derive(Copy, Clone)]
pub enum Coordinate<T> {
    Invalid,
    Valid(T),
}

impl<T> Coordinate<T> {
    /// Returns `true` if coordinate is invalid and `false` otherwise.
    pub fn is_invalid(&self) -> bool {
        match *self {
            Coordinate::Invalid => true,
            _ => false,
        }
    }

    /// Returns `true` if the coordinate is valid and `false` otherwise.
    pub fn is_valid(&self) -> bool {
        match *self {
            Coordinate::Invalid => false,
            _ => true,
        }
    }

    pub fn invalidate(&mut self) {
        *self = Coordinate::Invalid;
    }
}

/// A pair of coordinates.
pub type Pair = (usize, usize);

/// A trait for objects which have a position that can be expressed as coordinates.
pub trait Position<T> {
    /// Returns the current position's coordinates.
    fn pos(&self) -> Coordinate<T>;

    /// Sets the current position to the given coordinates and sets the last position's coordinates
    /// accordingly.
    fn set_pos(&mut self, newpos: Coordinate<T>);

    /// Invalidates the current position.
    fn invalidate_pos(&mut self);

    /// Returns the last position's coordinates.
    fn last_pos(&self) -> Coordinate<T>;

    /// Invalidates the last position.
    fn invalidate_last_pos(&mut self);
}

/// A cursor position.
pub struct Cursor {
    pos: Coordinate<Pair>,
    last_pos: Coordinate<Pair>,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            pos: Coordinate::Invalid,
            last_pos: Coordinate::Invalid,
        }
    }

    /// Checks whether the current and last coordinates are sequential and returns `true` if they
    /// are and `false` otherwise.
    pub fn is_seq(&self) -> bool {
        if let Coordinate::Valid((cx, cy)) =  self.pos {
            if let Coordinate::Valid((lx, ly)) = self.last_pos {
                (lx+1, ly) == (cx, cy)
            } else { false }
        } else { false }
    }

}

impl Position<Pair> for Cursor {
    fn pos(&self) -> Coordinate<Pair> {
        self.pos
    }

    fn set_pos(&mut self, newpos: Coordinate<Pair>) {
        self.last_pos = self.pos;
        self.pos = newpos;
    }

    fn invalidate_pos(&mut self) {
        self.pos.invalidate();
    }

    fn last_pos(&self) -> Coordinate<Pair> {
        self.last_pos
    }

    fn invalidate_last_pos(&mut self) {
        self.last_pos.invalidate();
    }
}
