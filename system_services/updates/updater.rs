use super::signature_verifier::SignatureVerifier;
use super::rollback::RollbackManager;

pub enum UpdateState {
    Idle,
    Downloading,
    Verifying,
    Installing,
    Failed,
    Completed,
}

pub struct Updater {
    pub state: UpdateState,
    verifier: SignatureVerifier,
    rollback: RollbackManager,
}

impl Updater {
    pub fn new() -> Self {
        Self {
            state: UpdateState::Idle,
            verifier: SignatureVerifier::new(),
            rollback: RollbackManager::new(),
        }
    }

    pub fn download_update(&mut self) {
        self.state = UpdateState::Downloading;

        // Placeholder: network fetch happens here
    }

    pub fn verify_update(&mut self, image: &[u8], signature: &[u8]) -> bool {
        self.state = UpdateState::Verifying;

        self.verifier.verify(image, signature)
    }

    pub fn install_update(&mut self) -> Result<(), &'static str> {
        if let UpdateState::Verifying | UpdateState::Downloading = self.state {
            self.state = UpdateState::Installing;

            // Save rollback snapshot before install
            self.rollback.snapshot();

            self.state = UpdateState::Completed;
            return Ok(());
        }

        self.state = UpdateState::Failed;
        Err("invalid update state")
    }

    pub fn fail_and_rollback(&mut self) {
        self.state = UpdateState::Failed;
        self.rollback.restore();
    }
}
