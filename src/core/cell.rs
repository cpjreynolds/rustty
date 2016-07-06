/// A single point on a terminal display.
///
/// A `Cell` contains a character and style.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Cell {
    ch: char,
    fg: Color,
    bg: Color,
    attrs: Attr,
}

impl Cell {
    /// Creates a new `Cell` with the given `char`, `Color`s and `Attr`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Color, Attr};
    ///
    /// let cell = Cell::new('x', Color::Default, Color::Green, Attr::empty());
    /// assert_eq!(cell.ch(), 'x');
    /// assert_eq!(cell.fg(), Color::Default);
    /// assert_eq!(cell.bg(), Color::Green);
    /// assert_eq!(cell.attrs(), Attr::empty());
    /// ```
    pub fn new(ch: char, fg: Color, bg: Color, attrs: Attr) -> Cell {
        Cell {
            ch: ch,
            fg: fg,
            bg: bg,
            attrs: attrs,
        }
    }

    /// Returns the `Cell`'s character.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Color, Attr};
    ///
    /// let cell = Cell::default();
    /// assert_eq!(cell.ch(), ' ');
    ///
    /// let cell = Cell::new('x', Color::Default, Color::Default, Attr::empty());
    /// assert_eq!(cell.ch(), 'x');
    /// ```
    pub fn ch(&self) -> char {
        self.ch
    }

    /// Sets the `Cell`'s character to the given `char`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::Cell;
    ///
    /// let mut cell = Cell::default();
    /// assert_eq!(cell.ch(), ' ');
    ///
    /// cell.set_ch('x');
    /// assert_eq!(cell.ch(), 'x');
    /// ```
    pub fn set_ch(&mut self, newch: char) -> &mut Cell {
        self.ch = newch;
        self
    }

    /// Returns the `Cell`'s foreground `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Color, Attr};
    ///
    /// let cell = Cell::new(' ', Color::Blue, Color::Default, Attr::empty());
    /// assert_eq!(cell.fg(), Color::Blue);
    /// ```
    pub fn fg(&self) -> Color {
        self.fg
    }

    /// Sets the `Cell`'s foreground `Color` to the given `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Color, Attr};
    ///
    /// let mut cell = Cell::default();
    /// assert_eq!(cell.fg(), Color::Default);
    ///
    /// cell.set_fg(Color::White);
    /// assert_eq!(cell.fg(), Color::White);
    /// ```
    pub fn set_fg(&mut self, newfg: Color) -> &mut Cell {
        self.fg = newfg;
        self
    }

    /// Returns the `Cell`'s background `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Color, Attr};
    ///
    /// let mut cell = Cell::new(' ', Color::Default, Color::Green, Attr::empty());
    /// assert_eq!(cell.bg(), Color::Green);
    /// ```
    pub fn bg(&self) -> Color {
        self.bg
    }

    /// Sets the `Cell`'s background `Color` to the given `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Color, Attr};
    ///
    /// let mut cell = Cell::default();
    /// assert_eq!(cell.bg(), Color::Default);
    ///
    /// cell.set_bg(Color::Black);
    /// assert_eq!(cell.bg(), Color::Black);
    /// ```
    pub fn set_bg(&mut self, newbg: Color) -> &mut Cell {
        self.bg = newbg;
        self
    }

    pub fn attrs(&self) -> Attr {
        self.attrs
    }

    pub fn set_attrs(&mut self, newattrs: Attr) -> &mut Cell {
        self.attrs = newattrs;
        self
    }
}

impl Default for Cell {
    /// Constructs a new `Cell` with a blank `char` and default `Color`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Color};
    ///
    /// let mut cell = Cell::default();
    /// assert_eq!(cell.ch(), ' ');
    /// assert_eq!(cell.fg(), Color::Default);
    /// assert_eq!(cell.bg(), Color::Default);
    /// ```
    fn default() -> Cell {
        Cell::new(' ', Color::Default, Color::Default, Attr::empty())
    }
}

/// The color of a `Cell`.
///
/// `Color::Default` represents the default color of the underlying terminal.
///
/// The eight basic colors may be used directly and correspond to 0x00..0x07 in the 8-bit (256)
/// color range; in addition, the eight basic colors coupled with `Attr::Bold` correspond to
/// 0x08..0x0f in the 8-bit color range.
///
/// `Color::Byte(..)` may be used to specify a color in the 8-bit range.
///
/// # Examples
///
/// ```
/// use rustty::Color;
///
/// // The default color.
/// let default = Color::Default;
///
/// // A basic color.
/// let red = Color::Red;
///
/// // An 8-bit color.
/// let fancy = Color::Byte(0x01);
///
/// // Basic colors are also 8-bit colors (but not vice-versa).
/// assert_eq!(red.as_byte(), fancy.as_byte())
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Byte(u8),
    Default,
}

impl Color {
    /// Returns the `u8` representation of the `Color`.
    pub fn as_byte(&self) -> u8 {
        match *self {
            Color::Black => 0x00,
            Color::Red => 0x01,
            Color::Green => 0x02,
            Color::Yellow => 0x03,
            Color::Blue => 0x04,
            Color::Magenta => 0x05,
            Color::Cyan => 0x06,
            Color::White => 0x07,
            Color::Byte(b) => b,
            Color::Default => panic!("Attempted to cast default color to u8"),
        }
    }
}

bitflags! {
    /// The attributes of a `Cell`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::Attr;
    ///
    /// // No attributes.
    /// let def = Attr::empty();
    ///
    /// // Single attribute.
    /// let bold = Attr::bold();
    ///
    /// // Combination.
    /// let comb = Attr::reverse() | Attr::underline();
    /// ```
    #[derive(Default)]
    pub flags Attr: u8 {
        const BOLD = 0b001,
        const UNDERLINE = 0b010,
        const REVERSE = 0b100,
    }
}

impl Attr {
    pub fn bold() -> Attr {
        BOLD
    }

    pub fn underline() -> Attr {
        UNDERLINE
    }

    pub fn reverse() -> Attr {
        REVERSE
    }
}
