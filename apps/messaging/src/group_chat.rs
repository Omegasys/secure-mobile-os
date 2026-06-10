use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct GroupMessage {
    pub id: u64,
    pub sender: String,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Clone)]
pub struct GroupChat {
    pub id: u64,
    pub name: String,
    pub members: HashSet<String>,
    pub messages: Vec<GroupMessage>,
}

pub struct GroupChatManager {
    groups: Vec<GroupChat>,
}

impl GroupChatManager {
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
        }
    }

    pub fn create_group(
        &mut self,
        name: String,
        members: Vec<String>,
    ) -> u64 {
        let id = timestamp();

        self.groups.push(GroupChat {
            id,
            name,
            members: members.into_iter().collect(),
            messages: Vec::new(),
        });

        id
    }

    pub fn send_message(
        &mut self,
        group_id: u64,
        sender: String,
        content: String,
    ) {
        if let Some(group) =
            self.groups.iter_mut().find(|g| g.id == group_id)
        {
            group.messages.push(GroupMessage {
                id: timestamp(),
                sender,
                content,
                timestamp: timestamp(),
            });
        }
    }

    pub fn get_group(
        &self,
        group_id: u64,
    ) -> Option<&GroupChat> {
        self.groups.iter().find(|g| g.id == group_id)
    }
}

fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
