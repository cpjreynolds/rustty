use ui::core::Widget;

pub trait Layout: Widget {
    fn align_elems(&mut self);
}
