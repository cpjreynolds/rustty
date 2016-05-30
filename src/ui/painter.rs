use core::cellbuffer::{CellAccessor, Cell};
use ui::layout::{HorizontalAlign, VerticalAlign};

#[derive(Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

pub trait Painter: CellAccessor {
    /// Prints a string at the specified position.
    ///
    /// This is a shorthand for setting each cell individually. `cell`'s style is going to be
    /// copied to each destination cell.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Terminal, Cell, Color, Attr};
    /// use rustty::ui::Painter;
    ///
    /// let mut term = Terminal::new().unwrap();
    /// let cell = Cell::with_style(Color::Default, Color::Red, Attr::Default);
    /// term.printline_with_cell(0, 0, "foobar", cell);
    /// ```
    fn printline_with_cell(&mut self, x: usize, y: usize, line: &str, cell: Cell) {
        let (cols, _) = self.size();
        for (index, ch) in line.chars().enumerate() {
            let current_x = x + index;
            if current_x >= cols {
                break;
            }
            match self.get_mut(current_x, y) {
                Some(c) => {
                    c.set_fg(cell.fg());
                    c.set_bg(cell.bg());
                    c.set_attrs(cell.attrs());
                    c.set_ch(ch);
                }
                None => {}
            }
        }
    }


    /// Prints a string at the specified position.
    ///
    /// Shorthand for `printline_with_cell(x, y, line, Cell::default())`.
    fn printline(&mut self, x: usize, y: usize, line: &str) {
        self.printline_with_cell(x, y, line, Cell::default());
    }

    /// Returns the proper x coord to align `line` in the specified `halign` alignment.
    ///
    /// `margin` is the number of characters we want to leave near the borders.
    fn halign_line(&self, line: &str, halign: HorizontalAlign, margin: usize) -> usize {
        let (cols, _) = self.size();
        match halign {
            HorizontalAlign::Left => margin,
            HorizontalAlign::Right => cols - line.chars().count() - margin - 1,
            HorizontalAlign::Middle => (cols - line.chars().count()) / 2,
        }
    }

    /// Returns the proper y coord to align `line` in the specified `valign` alignment.
    ///
    /// `margin` is the number of characters we want to leave near the borders.
    ///
    /// For now, the contents of line has no incidence whatsoever on the result, but when we
    /// support multi-line strings, it will, so we might as well stay consistent with
    /// `halign_line()`.
    #[allow(unused_variables)]
    fn valign_line(&self, line: &str, valign: VerticalAlign, margin: usize) -> usize {
        let (_, rows) = self.size();
        match valign {
            VerticalAlign::Top => margin,
            VerticalAlign::Bottom => rows - margin - 1,
            VerticalAlign::Middle => rows / 2,
        }
    }

    fn repeat_cell(&mut self,
                   x: usize,
                   y: usize,
                   orientation: Orientation,
                   count: usize,
                   cell: Cell) {
        for i in 0..count {
            let (ix, iy) = match orientation {
                Orientation::Horizontal => (x + i, y),
                Orientation::Vertical => (x, y + i),
            };
            match self.get_mut(ix, iy) {
                Some(c) => {
                    *c = cell;
                }
                None => (),
            };
        }
    }

    fn draw_box(&mut self) {
        let (cols, rows) = self.size();
        let corners = [(0, 0, '┌'),
                       (cols - 1, 0, '┐'),
                       (cols - 1, rows - 1, '┘'),
                       (0, rows - 1, '└')];
        for &(x, y, ch) in corners.iter() {
            self.get_mut(x, y).unwrap().set_ch(ch);
        }
        let lines = [(1, 0, cols - 2, Orientation::Horizontal, '─'),
                     (1, rows - 1, cols - 2, Orientation::Horizontal, '─'),
                     (0, 1, rows - 2, Orientation::Vertical, '│'),
                     (cols - 1, 1, rows - 2, Orientation::Vertical, '│')];
        for &(x, y, count, orientation, ch) in lines.iter() {
            let cell = Cell::with_char(ch);
            self.repeat_cell(x, y, orientation, count, cell);
        }
    }
}

impl<T: CellAccessor> Painter for T {}
