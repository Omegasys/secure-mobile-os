#[derive(Clone)]
pub struct AccessibilitySettings {
    pub screen_reader_enabled: bool,
    pub magnifier_enabled: bool,

    pub high_contrast_enabled: bool,

    pub large_text_enabled: bool,
    pub font_scale: f32,

    pub reduced_motion_enabled: bool,

    pub captions_enabled: bool,
}

impl AccessibilitySettings {
    pub fn new() -> Self {
        Self {
            screen_reader_enabled: false,
            magnifier_enabled: false,

            high_contrast_enabled: false,

            large_text_enabled: false,
            font_scale: 1.0,

            reduced_motion_enabled: false,

            captions_enabled: false,
        }
    }

    pub fn toggle_screen_reader(&mut self) {
        self.screen_reader_enabled =
            !self.screen_reader_enabled;
    }

    pub fn toggle_magnifier(&mut self) {
        self.magnifier_enabled =
            !self.magnifier_enabled;
    }

    pub fn toggle_high_contrast(&mut self) {
        self.high_contrast_enabled =
            !self.high_contrast_enabled;
    }

    pub fn toggle_large_text(&mut self) {
        self.large_text_enabled =
            !self.large_text_enabled;
    }

    pub fn toggle_reduced_motion(&mut self) {
        self.reduced_motion_enabled =
            !self.reduced_motion_enabled;
    }

    pub fn toggle_captions(&mut self) {
        self.captions_enabled =
            !self.captions_enabled;
    }

    pub fn set_font_scale(&mut self, scale: f32) {
        self.font_scale =
            scale.max(0.5).min(4.0);
    }

    pub fn print_status(&self) {
        println!("--- Accessibility Settings ---");

        println!(
            "Screen Reader: {}",
            self.screen_reader_enabled
        );

        println!(
            "Magnifier: {}",
            self.magnifier_enabled
        );

        println!(
            "High Contrast: {}",
            self.high_contrast_enabled
        );

        println!(
            "Large Text: {}",
            self.large_text_enabled
        );

        println!(
            "Font Scale: {}",
            self.font_scale
        );

        println!(
            "Reduced Motion: {}",
            self.reduced_motion_enabled
        );

        println!(
            "Captions: {}",
            self.captions_enabled
        );
    }
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self::new()
    }
}
