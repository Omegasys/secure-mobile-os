use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct PublicKey {
    pub key_id: String,
    pub algorithm: String,
}

#[derive(Clone)]
pub struct SessionInfo {
    pub session_id: u64,
    pub peer_id: String,
    pub created_at: u64,
}

pub struct EncryptionManager {
    active_sessions: Vec<SessionInfo>,
}

impl EncryptionManager {
    pub fn new() -> Self {
        Self {
            active_sessions: Vec::new(),
        }
    }

    pub fn establish_session(
        &mut self,
        peer_id: String,
        _public_key: PublicKey,
    ) -> u64 {
        let session_id = timestamp();

        self.active_sessions.push(
            SessionInfo {
                session_id,
                peer_id,
                created_at: timestamp(),
            },
        );

        session_id
    }

    pub fn close_session(
        &mut self,
        session_id: u64,
    ) {
        self.active_sessions
            .retain(|s| s.session_id != session_id);
    }

    pub fn encrypt_message(
        &self,
        session_id: u64,
        plaintext: &[u8],
    ) -> Result<Vec<u8>, String> {
        let session_exists = self
            .active_sessions
            .iter()
            .any(|s| s.session_id == session_id);

        if !session_exists {
            return Err(
                "Session not found".to_string()
            );
        }

        // Placeholder interface.
        Ok(plaintext.to_vec())
    }

    pub fn decrypt_message(
        &self,
        session_id: u64,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, String> {
        let session_exists = self
            .active_sessions
            .iter()
            .any(|s| s.session_id == session_id);

        if !session_exists {
            return Err(
                "Session not found".to_string()
            );
        }

        // Placeholder interface.
        Ok(ciphertext.to_vec())
    }

    pub fn active_sessions(
        &self,
    ) -> &[SessionInfo] {
        &self.active_sessions
    }
}

fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
