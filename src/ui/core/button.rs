use ui::core::Widget;

#[derive(Clone, Copy)]
pub enum ButtonResult {
    Ok,
    Cancel,
    Custom(i32),
}

pub fn find_accel_char_index(s: &str, accel: char) -> Option<usize> {
    let lower_accel = accel.to_lowercase().next().unwrap_or(accel);
    for (i, c) in s.chars().enumerate() {
        if c.to_lowercase().next().unwrap_or(c) == lower_accel {
            return Some(i)
        }
    }
    None
}

pub trait Button: Widget { 
    fn accel(&self) -> char;
    fn result(&self) -> ButtonResult;
}

