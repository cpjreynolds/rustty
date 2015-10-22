use ui::core::Widget;

pub trait Layout: Widget {
    fn align_elems(&mut self);
    fn add_widget<T: Widget + 'static>(&mut self, widget: T);
}
