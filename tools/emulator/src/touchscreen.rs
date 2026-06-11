```rust
#[derive(Debug)]
pub enum TouchEventType {
    Press,
    Move,
    Release,
}

#[derive(Debug)]
pub struct TouchEvent {
    pub x: u32,
    pub y: u32,
    pub event_type: TouchEventType,
}

pub struct VirtualTouchscreen {
    pub width: u32,
    pub height: u32,
}

impl VirtualTouchscreen {
    pub fn new(
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn process_touch(
        &self,
        event: TouchEvent,
    ) {
        println!(
            "Touch {:?} at ({}, {})",
            event.event_type,
            event.x,
            event.y
        );
    }

    pub fn is_valid_coordinate(
        &self,
        x: u32,
        y: u32,
    ) -> bool {
        x < self.width && y < self.height
    }
}
```
