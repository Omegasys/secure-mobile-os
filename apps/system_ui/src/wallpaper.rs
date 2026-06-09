#[derive(Clone)]
pub enum WallpaperType {
    Image,
    Animated,
    SolidColor,
}

#[derive(Clone)]
pub struct Wallpaper {
    pub id: u64,
    pub name: String,
    pub wallpaper_type: WallpaperType,

    pub path: String,

    pub enabled: bool,
}

pub struct WallpaperManager {
    home_wallpaper: Option<Wallpaper>,
    lock_wallpaper: Option<Wallpaper>,
}

impl WallpaperManager {
    pub fn new() -> Self {
        Self {
            home_wallpaper: None,
            lock_wallpaper: None,
        }
    }

    pub fn set_home_wallpaper(
        &mut self,
        wallpaper: Wallpaper,
    ) {
        self.home_wallpaper = Some(wallpaper);
    }

    pub fn set_lock_wallpaper(
        &mut self,
        wallpaper: Wallpaper,
    ) {
        self.lock_wallpaper = Some(wallpaper);
    }

    pub fn home_wallpaper(&self) -> Option<&Wallpaper> {
        self.home_wallpaper.as_ref()
    }

    pub fn lock_wallpaper(&self) -> Option<&Wallpaper> {
        self.lock_wallpaper.as_ref()
    }

    pub fn clear_home_wallpaper(&mut self) {
        self.home_wallpaper = None;
    }

    pub fn clear_lock_wallpaper(&mut self) {
        self.lock_wallpaper = None;
    }

    pub fn swap_wallpapers(&mut self) {
        std::mem::swap(
            &mut self.home_wallpaper,
            &mut self.lock_wallpaper,
        );
    }

    pub fn render_home(&self) {
        if let Some(wallpaper) = &self.home_wallpaper {
            println!(
                "Rendering home wallpaper: {}",
                wallpaper.name
            );
        }
    }

    pub fn render_lock(&self) {
        if let Some(wallpaper) = &self.lock_wallpaper {
            println!(
                "Rendering lock wallpaper: {}",
                wallpaper.name
            );
        }
    }
}

impl Default for WallpaperManager {
    fn default() -> Self {
        Self::new()
    }
}
