use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum SpamRisk {
    Safe,
    SuspectedSpam,
    ConfirmedSpam,
}

pub struct SpamFilter {
    blocked_numbers: HashSet<String>,
    spam_numbers: HashSet<String>,
}

impl SpamFilter {
    pub fn new() -> Self {
        Self {
            blocked_numbers: HashSet::new(),
            spam_numbers: HashSet::new(),
        }
    }

    pub fn block_number(
        &mut self,
        number: String,
    ) {
        self.blocked_numbers.insert(number);
    }

    pub fn unblock_number(
        &mut self,
        number: &str,
    ) {
        self.blocked_numbers.remove(number);
    }

    pub fn mark_as_spam(
        &mut self,
        number: String,
    ) {
        self.spam_numbers.insert(number);
    }

    pub fn is_blocked(
        &self,
        number: &str,
    ) -> bool {
        self.blocked_numbers.contains(number)
    }

    pub fn evaluate(
        &self,
        number: &str,
    ) -> SpamRisk {
        if self.blocked_numbers.contains(number) {
            SpamRisk::ConfirmedSpam
        } else if self.spam_numbers.contains(number) {
            SpamRisk::SuspectedSpam
        } else {
            SpamRisk::Safe
        }
    }

    pub fn should_allow_call(
        &self,
        number: &str,
    ) -> bool {
        !self.blocked_numbers.contains(number)
    }
}

impl Default for SpamFilter {
    fn default() -> Self {
        Self::new()
    }
}
