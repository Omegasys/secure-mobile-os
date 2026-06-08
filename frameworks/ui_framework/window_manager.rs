pub struct Window {
    pub id: u64,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
}

pub struct WindowManager {
    windows: [Option<Window>; 64],
}

impl WindowManager {
    pub const fn new() -> Self {
        Self {
            windows: [None; 64],
        }
    }

    pub fn create_window(&mut self, window: Window) {
        for slot in self.windows.iter_mut() {
            if slot.is_none() {
                *slot = Some(window);
                break;
            }
        }
    }

    pub fn move_window(&mut self, id: u64, x: i32, y: i32) {
        for w in self.windows.iter_mut().flatten() {
            if w.id == id {
                w.x = x;
                w.y = y;
            }
        }
    }
}
