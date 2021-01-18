//! Attempting to mimic Erlang processes:
//!  - process can exit normally or with error
//!  - linked processes will exit with same error
//!  - unless the linked process is set to trap exits, in which case
//!    it receives the error as a message
//!  - a process may be monitored, sending DOWN messages on exit
//! 
//! Issues to solve:
//!  - smallest wrapper around a process function to erase its type
//!  - determining ergonomic process definition and runtime strategy
//!  - api for sending/receiving strongly-typed user-defined messages
use std::future::Future;

// pub fn process<F, Fut>(process_fn: F) -> (Pid, impl Future<Output=Exit>)
// where
//     F: Fn(Mailbox) -> Fut + 'static,
//     Fut: Future<Output=Exit> + Send + 'static
// {   
//     let (pid, mailbox) = create();
//     (pid, process_fn(mailbox))
// }

/// Spawns a new process from the provided process function
pub fn spawn<F, Fut>(process_fn: F) -> Pid
where
    F: Fn(Process) -> Fut,
    Fut: Future<Output=Exit> + Send + 'static
{
    // keep code minimal here to reduce compiled binary size
    let process = Process::new();
    let pid = process.pid.clone();
    smol::spawn((process_fn)(process)).detach();

    pid
}

pub enum Exit {
    Normal
}

pub struct Process {
    pub pid: Pid,           // storing Pid allows sending to self
    pub mailbox: Mailbox,   // incoming messages
}

impl Process {
    pub fn new() -> Self {
        let channel = smol::channel::unbounded();
    
        Process {
            pid: Pid::new(channel.0),
            mailbox: Mailbox::new(channel.1),
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
#[derive(Clone)]
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

    /// Receives a `Msg`, or `None`: indicating that the channel is closed.
    /// TODO: recv() should be infallible
    pub async fn recv(&self) -> Option<Msg> {
        self.receiver.recv().await.ok()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn spawn_process() {
        spawn(|_process| async move {
            Exit::Normal
        });
    }

    // TODO: Lifetime problems with receiving...
    #[test]
    fn receive_message() {
        let pid = spawn(|process| async move {
            match process.mailbox.recv().await {
                _ => Exit::Normal 
            }
        });
    }
}
