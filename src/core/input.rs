/// An input event.
///
/// An `Event` represents a single event from the underying terminal. At the moment no further
/// processing is done on events and raw escape sequences will also be passed as `Key`s.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Event {
    Key(char),
}
