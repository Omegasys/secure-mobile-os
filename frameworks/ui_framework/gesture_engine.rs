pub enum GestureType {
    Tap,
    Swipe,
    Pinch,
    LongPress,
}

pub struct GestureEvent {
    pub gesture: GestureType,
    pub x: i32,
    pub y: i32,
}

pub struct GestureEngine;

impl GestureEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn detect(&self, raw_input: &[u8]) -> Option<GestureEvent> {
        let _ = raw_input;

        // placeholder gesture detection logic
        Some(GestureEvent {
            gesture: GestureType::Tap,
            x: 0,
            y: 0,
        })
    }
}
