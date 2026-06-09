#[derive(Clone)]
pub enum BrightnessMode {
    Manual,
    Adaptive,
}

#[derive(Clone)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

#[derive(Clone)]
pub struct DisplaySettings {
    pub brightness: u8,
    pub brightness_mode: BrightnessMode,

    pub refresh_rate: u16,

    pub theme_mode: ThemeMode,
    pub auto_rotate: bool,

    pub high_contrast: bool,
    pub font_scale: f32,
}

impl DisplaySettings {
    pub fn new() -> Self {
        Self {
            brightness: 75,
            brightness_mode: BrightnessMode::Adaptive,

            refresh_rate: 120,

            theme_mode: ThemeMode::System,
            auto_rotate: true,

            high_contrast: false,
            font_scale: 1.0,
        }
    }

    pub fn set_brightness(&mut self, value: u8) {
        self.brightness = value.min(100);
    }

    pub fn toggle_auto_rotate(&mut self) {
        self.auto_rotate = !self.auto_rotate;
    }

    pub fn set_refresh_rate(&mut self, rate: u16) {
        self.refresh_rate = rate;
    }

    pub fn set_theme(&mut self, theme: ThemeMode) {
        self.theme_mode = theme;
    }

    pub fn toggle_high_contrast(&mut self) {
        self.high_contrast = !self.high_contrast;
    }

    pub fn set_font_scale(&mut self, scale: f32) {
        self.font_scale = scale.max(0.5).min(3.0);
    }

    pub fn print_status(&self) {
        println!("--- Display Settings ---");
        println!("Brightness: {}", self.brightness);
        println!("Refresh Rate: {}Hz", self.refresh_rate);
        println!("Auto Rotate: {}", self.auto_rotate);
        println!("High Contrast: {}", self.high_contrast);
        println!("Font Scale: {}", self.font_scale);

        match self.theme_mode {
            ThemeMode::Light => println!("Theme: Light"),
            ThemeMode::Dark => println!("Theme: Dark"),
            ThemeMode::System => println!("Theme: System"),
        }
    }
}
