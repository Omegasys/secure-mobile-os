#[derive(Clone)]
pub enum AuthenticationMethod {
    Password,
    Pin,
    Fingerprint,
    FaceRecognition,
    HardwareToken,
}

#[derive(Clone)]
pub struct UserAccount {
    pub id: u64,
    pub username: String,

    pub admin: bool,
    pub enabled: bool,

    pub auth_method: AuthenticationMethod,
}

pub struct AccountManager {
    users: Vec<UserAccount>,
}

impl AccountManager {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
        }
    }

    pub fn add_user(
        &mut self,
        user: UserAccount,
    ) {
        self.users.push(user);
    }

    pub fn disable_user(
        &mut self,
        username: &str,
    ) {
        for user in &mut self.users {
            if user.username == username {
                user.enabled = false;
            }
        }
    }

    pub fn enable_user(
        &mut self,
        username: &str,
    ) {
        for user in &mut self.users {
            if user.username == username {
                user.enabled = true;
            }
        }
    }

    pub fn remove_user(
        &mut self,
        username: &str,
    ) {
        self.users
            .retain(|u| u.username != username);
    }

    pub fn user_count(&self) -> usize {
        self.users.len()
    }

    pub fn users(&self) -> &[UserAccount] {
        &self.users
    }

    pub fn print_accounts(&self) {
        println!("--- Accounts ---");

        for user in &self.users {
            println!(
                "{} | admin={} | enabled={}",
                user.username,
                user.admin,
                user.enabled
            );
        }
    }
}

impl Default for AccountManager {
    fn default() -> Self {
        Self::new()
    }
}
