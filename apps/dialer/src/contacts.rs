#[derive(Clone)]
pub struct Contact {
    pub id: u64,
    pub name: String,
    pub phone_number: String,
}

pub struct ContactBook {
    contacts: Vec<Contact>,
}

impl ContactBook {
    pub fn new() -> Self {
        Self {
            contacts: Vec::new(),
        }
    }

    pub fn add_contact(
        &mut self,
        contact: Contact,
    ) {
        self.contacts.push(contact);
    }

    pub fn remove_contact(
        &mut self,
        id: u64,
    ) {
        self.contacts.retain(|c| c.id != id);
    }

    pub fn find_by_name(
        &self,
        name: &str,
    ) -> Option<&Contact> {
        self.contacts
            .iter()
            .find(|c| c.name.eq_ignore_ascii_case(name))
    }

    pub fn find_by_number(
        &self,
        number: &str,
    ) -> Option<&Contact> {
        self.contacts
            .iter()
            .find(|c| c.phone_number == number)
    }

    pub fn list_contacts(&self) {
        println!("--- Contacts ---");

        for contact in &self.contacts {
            println!(
                "{} -> {}",
                contact.name,
                contact.phone_number
            );
        }
    }

    pub fn count(&self) -> usize {
        self.contacts.len()
    }
}
