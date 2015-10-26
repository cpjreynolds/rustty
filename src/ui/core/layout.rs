use ui::core::{Widget, ButtonResult};
use std::collections::HashMap;

pub trait Layout: Widget {
    fn align_elems(&mut self);
    fn forward_keys(&mut self, key_map: &mut HashMap<char, ButtonResult>);
}
