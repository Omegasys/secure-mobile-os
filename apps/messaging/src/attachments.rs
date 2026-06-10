use std::path::PathBuf;

#[derive(Clone)]
pub enum AttachmentType {
    Image,
    Video,
    Audio,
    Document,
    Archive,
    Other,
}

#[derive(Clone)]
pub struct Attachment {
    pub id: u64,
    pub filename: String,
    pub file_path: PathBuf,
    pub size_bytes: u64,
    pub attachment_type: AttachmentType,
}

pub struct AttachmentManager {
    attachments: Vec<Attachment>,
}

impl AttachmentManager {
    pub fn new() -> Self {
        Self {
            attachments: Vec::new(),
        }
    }

    pub fn add_attachment(
        &mut self,
        attachment: Attachment,
    ) {
        self.attachments.push(attachment);
    }

    pub fn remove_attachment(
        &mut self,
        id: u64,
    ) {
        self.attachments.retain(|a| a.id != id);
    }

    pub fn get_attachment(
        &self,
        id: u64,
    ) -> Option<&Attachment> {
        self.attachments.iter().find(|a| a.id == id)
    }

    pub fn list_attachments(
        &self,
    ) -> &[Attachment] {
        &self.attachments
    }
}
