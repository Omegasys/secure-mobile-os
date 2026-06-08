#[derive(Clone, Copy)]
pub enum InputEventType {
    KeyPress,
    KeyRelease,
    TouchDown,
    TouchMove,
    TouchUp,
}

pub struct InputEvent {
    pub event_type: InputEventType,
    pub x: i32,
    pub y: i32,
    pub keycode: u16,
}

pub struct InputDriver;

impl InputDriver {
    pub const fn new() -> Self {
        Self
    }

    pub fn poll(&self) -> Option<InputEvent> {
        None
    }
}
