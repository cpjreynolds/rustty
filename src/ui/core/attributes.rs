#[derive(Clone)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

#[derive(Clone)]
pub enum HorizontalAlign {
    Left,
    Middle,
    Right,
}

#[derive(Clone, Copy)]
pub enum ButtonResult {
    Ok,
    Cancel,
    Custom(i32),
}
