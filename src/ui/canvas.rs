use core::position::{Size, HasSize, Pos, HasPosition};
use core::cellbuffer::{Cell, CellAccessor};

use ui::core::{
    Alignable,
    HorizontalAlign,
    VerticalAlign,
    Widget,
    Frame,
    Painter
};

/// A logical clone of [Frame](core/frame/struct.Frame.html) that exposes backend
/// functionality for users without breaking the API rules
///
/// # Examples
///
/// ```ignore
/// use rustty::ui::Canvas;
///
/// let mut canvas = Canvas::new(60, 10);
///
/// let (rows, cols) = canvas.size();
/// // Set the entire canvas to '-' character
/// for i in 0..cols*rows {
///     let y = i / cols;
///     let x = i % cols;
///     let mut cell = canvas.get_mut(x, y).unwrap();
///     cell.set_ch('-');
/// }
/// ```
///
pub struct Canvas {
    frame: Frame
}

impl Canvas {
    /// Constructs a new `Canvas` object *cols* wide by *rows* high
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::ui::Canvas;
    ///
    /// let mut canvas = Canvas::new(60, 10); 
    /// ```
    ///
    pub fn new(cols: usize, rows: usize) -> Canvas {
        Canvas {
            frame: Frame::new(cols, rows)
        }
    }

    /// Returns the size of the canvas
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::ui::Canvas;
    ///
    /// let mut canvas = Canvas::new(60, 10);
    ///
    /// assert_eq!(canvas.size(), (60, 10));
    /// ```
    ///
    pub fn size(&self) -> Size {
        self.frame.size()
    }

    /// Returns a reference to the vector of cells
    /// that lie within the canvas
    pub fn cellvec(&self) -> &Vec<Cell> {
        self.frame.cellvec()
    }

    /// Returns a mutable reference to the vector
    /// of cells that lie within the canvas
    pub fn cellvec_mut(&mut self) -> &mut Vec<Cell> {
        self.frame.cellvec_mut()
    }

    /// Clears the canvas with a *blank* [Cell](../struct.Cell.html)
    pub fn clear(&mut self, blank: Cell) {
        self.frame.clear(blank);
    }

    /// Converts a position on the screen to the relative coordinate 
    /// within the Canvas cell buffer
    pub fn pos_to_index(&self, x: usize, y: usize) -> Option<usize> {
        self.frame.pos_to_index(x, y)
    }

    /// Returns a reference to the cell at the specified position (*x*,*y*),
    /// in the form of an *Option*. If no cell exists at that position,
    /// then *None*
    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.frame.get(x, y)
    }

    /// Returns a mutable reference to the cell at the specified position 
    /// (*x*, *y*), in the form of an *Option*. If no cell exists at that
    /// position, then *None*
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.frame.get_mut(x, y)
    }

    /// The location of the canvas
    pub fn origin(&self) -> Pos {
        self.frame.origin()
    }

    /// Mantually sets the location of the canvas
    pub fn set_origin(&mut self, new_origin: Pos) {
        self.frame.set_origin(new_origin);
    }
}

impl Widget for Canvas {
    fn draw(&mut self, parent: &mut CellAccessor) {
        self.frame.draw_into(parent);
    }
    
    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
            margin: (usize, usize)) {
        self.frame.align(parent, halign, valign, margin);
    }

    fn resize(&mut self, new_size: Size) {
        self.frame.resize(new_size);
    }

    fn draw_box(&mut self) {
        self.frame.draw_box();
    }

    fn frame(&self) -> &Frame {
        &self.frame
    }

    fn frame_mut(&mut self) -> &mut Frame {
        &mut self.frame
    }
}

