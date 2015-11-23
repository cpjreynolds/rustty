use ui::core::{ButtonResult, Widget};

/// Helper function for finding the location of the key in the string
/// that is to be bolded
pub fn find_accel_char_index(s: &str, accel: char) -> Option<usize> {
    let lower_accel = accel.to_lowercase().next().unwrap_or(accel);
    for (i, c) in s.chars().enumerate() {
        if c.to_lowercase().next().unwrap_or(c) == lower_accel {
            return Some(i)
        }
    }
    None
}

/// Trait used when designing new buttons. All buttons implement some 
/// key that is recorded, and returns some result that can be matched
/// to run some action
pub trait Button: Widget { 
    /// Return the char that is acting as the key in the Button
    fn accel(&self) -> char;
    /// Return the `ButtonResult` which would be returned if the
    /// key is detected
    fn result(&self) -> ButtonResult;
    /// If a button is to do some special action upon being pressed,
    /// then this function will do so. StdButton for example does
    /// nothing when pressed, while CheckButton changes it's ballot
    fn pressed(&mut self) { }
    /// If a button has a state involved, e.g. needs to keep track 
    /// of a certain event, this function will return the state of
    /// the button.
    fn state(&self) -> bool { false }
}

