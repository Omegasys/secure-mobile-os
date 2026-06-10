#[derive(Default)]
pub struct Keypad {
    current_number: String,
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            current_number: String::new(),
        }
    }

    pub fn press_digit(&mut self, digit: char) {
        if digit.is_ascii_digit() {
            self.current_number.push(digit);
        }
    }

    pub fn press_star(&mut self) {
        self.current_number.push('*');
    }

    pub fn press_hash(&mut self) {
        self.current_number.push('#');
    }

    pub fn press_plus(&mut self) {
        if self.current_number.is_empty() {
            self.current_number.push('+');
        }
    }

    pub fn backspace(&mut self) {
        self.current_number.pop();
    }

    pub fn clear(&mut self) {
        self.current_number.clear();
    }

    pub fn get_number(&self) -> &str {
        &self.current_number
    }

    pub fn is_empty(&self) -> bool {
        self.current_number.is_empty()
    }
}
