pub enum JournalEntry {
    Create,
    Write,
    Delete,
}

pub struct Journal {
    log: [Option<JournalEntry>; 1024],
    index: usize,
}

impl Journal {
    pub const fn new() -> Self {
        Self {
            log: [None; 1024],
            index: 0,
        }
    }

    pub fn record(&mut self, entry: JournalEntry) {
        if self.index < 1024 {
            self.log[self.index] = Some(entry);
            self.index += 1;
        }
    }
}
