use ui::core::{Widget, ButtonResult};
use std::collections::HashMap;

/// Specialized version of a widget that implements an alignment function
/// and method for forwarding keys to the parent widgets key map. 
pub trait Layout: Widget {
    fn align_elems(&mut self);
    fn forward_keys(&mut self, key_map: &mut HashMap<char, ButtonResult>);
}
