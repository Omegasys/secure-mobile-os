#[derive(Clone)]
pub struct DeveloperOptions {
    pub developer_mode: bool,
    pub usb_debugging: bool,
    pub wireless_debugging: bool,

    pub show_fps: bool,
    pub show_layout_bounds: bool,
    pub show_touch_points: bool,

    pub verbose_logging: bool,

    pub allow_unsigned_apps: bool,
}

impl DeveloperOptions {
    pub fn new() -> Self {
        Self {
            developer_mode: false,
            usb_debugging: false,
            wireless_debugging: false,

            show_fps: false,
            show_layout_bounds: false,
            show_touch_points: false,

            verbose_logging: false,

            allow_unsigned_apps: false,
        }
    }

    pub fn toggle_developer_mode(&mut self) {
        self.developer_mode = !self.developer_mode;
    }

    pub fn toggle_usb_debugging(&mut self) {
        self.usb_debugging = !self.usb_debugging;
    }

    pub fn toggle_wireless_debugging(&mut self) {
        self.wireless_debugging =
            !self.wireless_debugging;
    }

    pub fn toggle_fps_overlay(&mut self) {
        self.show_fps = !self.show_fps;
    }

    pub fn toggle_layout_bounds(&mut self) {
        self.show_layout_bounds =
            !self.show_layout_bounds;
    }

    pub fn toggle_touch_points(&mut self) {
        self.show_touch_points =
            !self.show_touch_points;
    }

    pub fn toggle_verbose_logging(&mut self) {
        self.verbose_logging =
            !self.verbose_logging;
    }

    pub fn toggle_unsigned_apps(&mut self) {
        self.allow_unsigned_apps =
            !self.allow_unsigned_apps;
    }

    pub fn print_status(&self) {
        println!("--- Developer Options ---");
        println!("Developer Mode: {}", self.developer_mode);
        println!("USB Debugging: {}", self.usb_debugging);
        println!(
            "Wireless Debugging: {}",
            self.wireless_debugging
        );
        println!("FPS Overlay: {}", self.show_fps);
        println!(
            "Layout Bounds: {}",
            self.show_layout_bounds
        );
        println!(
            "Touch Points: {}",
            self.show_touch_points
        );
        println!(
            "Verbose Logging: {}",
            self.verbose_logging
        );
        println!(
            "Allow Unsigned Apps: {}",
            self.allow_unsigned_apps
        );
    }
}

impl Default for DeveloperOptions {
    fn default() -> Self {
        Self::new()
    }
}
