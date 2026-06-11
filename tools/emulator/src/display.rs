```rust
#[derive(Debug)]
pub struct VirtualDisplay {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
}

impl VirtualDisplay {
    pub fn new(
        width: u32,
        height: u32,
        refresh_rate: u32,
    ) -> Self {
        Self {
            width,
            height,
            refresh_rate,
        }
    }

    pub fn render_frame(&self) {
        println!(
            "Rendering frame at {}x{}",
            self.width,
            self.height
        );
    }

    pub fn print_info(&self) {
        println!("Display");
        println!(
            "Resolution: {}x{}",
            self.width,
            self.height
        );
        println!(
            "Refresh Rate: {} Hz",
            self.refresh_rate
        );
    }
}
```
