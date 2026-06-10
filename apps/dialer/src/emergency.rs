#[derive(Clone)]
pub struct EmergencyContact {
    pub name: String,
    pub phone_number: String,
}

pub struct EmergencyManager {
    emergency_numbers: Vec<String>,
    emergency_contacts: Vec<EmergencyContact>,
}

impl EmergencyManager {
    pub fn new() -> Self {
        Self {
            emergency_numbers: vec![
                "911".to_string(),
                "112".to_string(),
            ],
            emergency_contacts: Vec::new(),
        }
    }

    pub fn add_contact(
        &mut self,
        contact: EmergencyContact,
    ) {
        self.emergency_contacts.push(contact);
    }

    pub fn is_emergency_number(
        &self,
        number: &str,
    ) -> bool {
        self.emergency_numbers
            .iter()
            .any(|n| n == number)
    }

    pub fn place_emergency_call(
        &self,
        number: &str,
    ) -> Result<(), String> {
        if self.is_emergency_number(number) {
            println!(
                "Emergency call requested: {}",
                number
            );

            // Future:
            // modem_hal::emergency_call(number);

            Ok(())
        } else {
            Err(
                "Not a recognized emergency number"
                    .to_string(),
            )
        }
    }

    pub fn contacts(
        &self,
    ) -> &[EmergencyContact] {
        &self.emergency_contacts
    }
}

impl Default for EmergencyManager {
    fn default() -> Self {
        Self::new()
    }
}
