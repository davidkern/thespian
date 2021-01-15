//! Attempting to mimic Erlang processes:
//!  - process can exit normally or with error
//!  - linked process will exit with same error
//!  - unless the linked process is set to trap exits
//!  - a process may be monitored, sending DOWN messages on exit
use std::future::Future;

/// Spawns a new process from the provided process function
pub fn spawn<F, Fut>(process_fn: F) -> Pid
where
    F: Fn(Mailbox) -> Fut + 'static,
    Fut: Future<Output=Exit> + Send + 'static
{
    // keep code minimal here to reduce compiled binary size
    let (pid, mailbox) = create();
    smol::spawn((process_fn)(mailbox)).detach();

    pid
}

/// Creates (Pid, Mailbox) for the spawn function
fn create() -> (Pid, Mailbox) {
    let (sender, receiver) = smol::channel::unbounded();
    (Pid::new(sender), Mailbox::new(receiver))
}

pub enum Exit {
    Normal
}

pub struct Process {
    pub mailbox: Mailbox,
}

impl Process {
    pub fn new(mailbox: Mailbox) -> Self {
        Process {
            mailbox,
        }
    }
}

/// Run forever until killed
pub async fn pending(ctx: &Process) -> Exit {
    // Define process state here

    // Normal process message loop
    while let Some(_msg) = ctx.mailbox.recv().await {
        // Process messages here
    }

    Exit::Normal
}

/// Some kind of message
pub struct Msg;

/// Process Handle
pub struct Pid {
    sender: smol::channel::Sender<Msg>,
}

impl Pid {
    pub fn new(sender: smol::channel::Sender<Msg>) -> Self {
        Pid {
            sender,
        }
    }
}

/// In some kind of mailbox
pub struct Mailbox {
    receiver: smol::channel::Receiver<Msg>,
}

impl Mailbox {
    pub fn new(receiver: smol::channel::Receiver<Msg>) -> Self {
        Mailbox {
            receiver
        }
    }

    pub async fn recv(&self) -> Option<Msg> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn kill_process() {
    }
}