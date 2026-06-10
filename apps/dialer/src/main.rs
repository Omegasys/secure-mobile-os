mod keypad;
mod contacts;

use keypad::Keypad;
use contacts::{ContactBook, Contact};

pub struct DialerApp {
    keypad: Keypad,
    contacts: ContactBook,
}

impl DialerApp {
    pub fn new() -> Self {
        Self {
            keypad: Keypad::new(),
            contacts: ContactBook::new(),
        }
    }

    pub fn dial_number(&self, number: &str) {
        println!("Dialing {}...", number);

        // Future integration:
        // modem_hal::place_call(number)
    }

    pub fn show_contacts(&self) {
        self.contacts.list_contacts();
    }
}

fn main() {
    let mut app = DialerApp::new();

    app.contacts.add_contact(Contact {
        id: 1,
        name: "Alice".to_string(),
        phone_number: "+15551234567".to_string(),
    });

    app.show_contacts();

    app.dial_number("+15551234567");
}
