#[derive(Clone)]
pub struct Participant {
    pub id: u64,
    pub phone_number: String,
    pub muted: bool,
}

pub struct ConferenceCall {
    pub id: u64,
    participants: Vec<Participant>,
    active: bool,
}

impl ConferenceCall {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            participants: Vec::new(),
            active: true,
        }
    }

    pub fn add_participant(
        &mut self,
        participant: Participant,
    ) {
        self.participants.push(participant);
    }

    pub fn remove_participant(
        &mut self,
        participant_id: u64,
    ) {
        self.participants
            .retain(|p| p.id != participant_id);
    }

    pub fn mute_participant(
        &mut self,
        participant_id: u64,
    ) {
        if let Some(p) = self
            .participants
            .iter_mut()
            .find(|p| p.id == participant_id)
        {
            p.muted = true;
        }
    }

    pub fn unmute_participant(
        &mut self,
        participant_id: u64,
    ) {
        if let Some(p) = self
            .participants
            .iter_mut()
            .find(|p| p.id == participant_id)
        {
            p.muted = false;
        }
    }

    pub fn participant_count(
        &self,
    ) -> usize {
        self.participants.len()
    }

    pub fn participants(
        &self,
    ) -> &[Participant] {
        &self.participants
    }

    pub fn end_call(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}
