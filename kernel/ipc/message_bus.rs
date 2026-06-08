use super::channels::Message;

pub const MAX_SUBSCRIBERS: usize = 256;

pub struct MessageBus {
    subscribers: [Option<usize>; MAX_SUBSCRIBERS],
}

impl MessageBus {
    pub const fn new() -> Self {
        Self {
            subscribers: [None; MAX_SUBSCRIBERS],
        }
    }

    pub fn subscribe(
        &mut self,
        pid: usize,
    ) -> Result<(), &'static str> {
        for slot in self.subscribers.iter_mut() {
            if slot.is_none() {
                *slot = Some(pid);
                return Ok(());
            }
        }

        Err("subscriber table full")
    }

    pub fn broadcast(&self, _message: &Message) {
        for subscriber in self.subscribers.iter() {
            if subscriber.is_some() {
                // Dispatch event
            }
        }
    }
}
