pub const CHANNEL_BUFFER_SIZE: usize = 1024;

pub struct Message {
    pub sender_pid: usize,
    pub receiver_pid: usize,
    pub length: usize,
    pub payload: [u8; CHANNEL_BUFFER_SIZE],
}

pub struct Channel {
    id: usize,
}

impl Channel {
    pub const fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn send(
        &self,
        _message: Message,
    ) -> Result<(), &'static str> {
        Ok(())
    }

    pub fn receive(&self) -> Option<Message> {
        None
    }
}
