use core::panel::{Panel, Draw};
use core::symbol;

pub struct Border {
    tl: char,
    tr: char,
    bl: char,
    br: char,
    h_edge: char,
    v_edge: char,
}

impl Draw for Border {
    fn draw(&self, x: usize, y: usize, target: &mut Panel) {
        let cols = target.cols();
        let rows = target.rows();
        let lenx = cols - x;
        let leny = rows - y;

        // top line.
        for (i, cell) in target.chunks_mut(cols)
            .skip(y)
            .take(1)
            .flat_map(|line| line.iter_mut().skip(x).take(lenx))
            .enumerate() {

            if i == 0 {
                cell.set_ch(self.tl);
            } else if i == (lenx - 1) {
                cell.set_ch(self.tr);
            } else {
                cell.set_ch(self.h_edge);
            }
        }

        // middle lines.
        for (i, cell) in target.chunks_mut(cols)
            .skip(y + 1)
            .take(leny)
            .flat_map(|line| line.iter_mut().skip(x).take(lenx))
            .enumerate() {

            if i % cols == 0 || i % cols == (lenx - 1) {
                cell.set_ch(self.v_edge);
            }
        }

        // bottom line.
        for (i, cell) in target.chunks_mut(cols)
            .skip(y + leny - 1)
            .take(1)
            .flat_map(|line| line.iter_mut().skip(x).take(lenx))
            .enumerate() {

            if i == 0 {
                cell.set_ch(self.bl);
            } else if i == (lenx - 1) {
                cell.set_ch(self.br);
            } else {
                cell.set_ch(self.h_edge);
            }
        }
    }
}

impl Default for Border {
    fn default() -> Border {
        Border {
            tl: symbol::BOX_H_DN_RT,
            tr: symbol::BOX_H_DN_LT,
            bl: symbol::BOX_H_UP_RT,
            br: symbol::BOX_H_UP_LT,
            h_edge: symbol::BOX_H_HORIZ,
            v_edge: symbol::BOX_H_VERT,
        }
    }
}
