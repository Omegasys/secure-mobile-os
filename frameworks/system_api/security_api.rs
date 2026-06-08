pub struct SecurityAPI {
    pub camera_allowed: bool,
    pub microphone_allowed: bool,
    pub location_allowed: bool,
}

impl SecurityAPI {
    pub fn new() -> Self {
        Self {
            camera_allowed: false,
            microphone_allowed: false,
            location_allowed: false,
        }
    }

    pub fn grant_camera(&mut self) {
        self.camera_allowed = true;
    }

    pub fn grant_microphone(&mut self) {
        self.microphone_allowed = true;
    }

    pub fn grant_location(&mut self) {
        self.location_allowed = true;
    }

    pub fn revoke_all(&mut self) {
        self.camera_allowed = false;
        self.microphone_allowed = false;
        self.location_allowed = false;
    }

    pub fn status(&self) -> (bool, bool, bool) {
        (
            self.camera_allowed,
            self.microphone_allowed,
            self.location_allowed,
        )
    }
}
