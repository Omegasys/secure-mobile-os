#[repr(u64)]
pub enum Syscall {
    Exit = 0,
    Read = 1,
    Write = 2,
    Open = 3,
    Close = 4,
    Spawn = 5,
}

pub fn handle_syscall(
    syscall: Syscall,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> usize {
    match syscall {
        Syscall::Exit => 0,
        Syscall::Read => {
            let _ = (arg0, arg1, arg2);
            0
        }
        Syscall::Write => {
            let _ = (arg0, arg1, arg2);
            0
        }
        Syscall::Open => 0,
        Syscall::Close => 0,
        Syscall::Spawn => 0,
    }
}
