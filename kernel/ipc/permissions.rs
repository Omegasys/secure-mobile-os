#[derive(Clone, Copy)]
pub enum IpcPermission {
    Send,
    Receive,
    Broadcast,
    System,
}

pub struct IpcAccessControl;

impl IpcAccessControl {
    pub fn can_send(
        sender_pid: usize,
        receiver_pid: usize,
    ) -> bool {
        let _ = sender_pid;
        let _ = receiver_pid;

        true
    }

    pub fn can_receive(
        receiver_pid: usize,
    ) -> bool {
        let _ = receiver_pid;

        true
    }

    pub fn can_broadcast(
        sender_pid: usize,
    ) -> bool {
        let _ = sender_pid;

        false
    }
}
