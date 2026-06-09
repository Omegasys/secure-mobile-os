#[derive(Clone, Copy)]
pub enum ThemeMode {
    Light,
    Dark,
    Automatic,
}

#[derive(Clone)]
pub struct Theme {
    pub id: u64,
    pub name: String,

    pub primary_color: String,
    pub secondary_color: String,

    pub icon_pack: String,
    pub font_family: String,
}

pub struct ThemeManager {
    current_theme: Option<Theme>,
    mode: ThemeMode,

    high_contrast: bool,
    large_text: bool,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            current_theme: None,
            mode: ThemeMode::Automatic,

            high_contrast: false,
            large_text: false,
        }
    }

    pub fn apply_theme(&mut self, theme: Theme) {
        self.current_theme = Some(theme);
    }

    pub fn current_theme(&self) -> Option<&Theme> {
        self.current_theme.as_ref()
    }

    pub fn set_mode(&mut self, mode: ThemeMode) {
        self.mode = mode;
    }

    pub fn mode(&self) -> ThemeMode {
        self.mode
    }

    pub fn enable_high_contrast(&mut self) {
        self.high_contrast = true;
    }

    pub fn disable_high_contrast(&mut self) {
        self.high_contrast = false;
    }

    pub fn enable_large_text(&mut self) {
        self.large_text = true;
    }

    pub fn disable_large_text(&mut self) {
        self.large_text = false;
    }

    pub fn high_contrast(&self) -> bool {
        self.high_contrast
    }

    pub fn large_text(&self) -> bool {
        self.large_text
    }

    pub fn reload_ui(&self) {
        println!("Reloading UI theme...");
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}
