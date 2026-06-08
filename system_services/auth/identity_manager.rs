pub struct Identity {
    pub user_id: u64,
    pub public_key: [u8; 32],
}

pub struct IdentityManager {
    current: Option<Identity>,
}

impl IdentityManager {
    pub const fn new() -> Self {
        Self { current: None }
    }

    pub fn set_identity(&mut self, identity: Identity) {
        self.current = Some(identity);
    }

    pub fn get_identity(&self) -> Option<&Identity> {
        self.current.as_ref()
    }

    pub fn clear(&mut self) {
        self.current = None;
    }
}
