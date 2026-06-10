#[derive(Clone)]
pub struct Contact {
    pub id: u64,
    pub display_name: String,
    pub phone_number: String,
}

pub struct ContactDirectory {
    contacts: Vec<Contact>,
}

impl ContactDirectory {
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

    pub fn find_by_number(
        &self,
        number: &str,
    ) -> Option<&Contact> {
        self.contacts
            .iter()
            .find(|c| c.phone_number == number)
    }

    pub fn find_by_name(
        &self,
        name: &str,
    ) -> Option<&Contact> {
        self.contacts
            .iter()
            .find(|c| {
                c.display_name
                    .eq_ignore_ascii_case(name)
            })
    }

    pub fn all_contacts(
        &self,
    ) -> &[Contact] {
        &self.contacts
    }
}
