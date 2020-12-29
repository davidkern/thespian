use std::future::Future;
use std::marker::Send;

#[cfg(test)]
mod test {
    use super::*;

    pub enum Foo {
        A,
        B,
        Stop,
    }
    
    impl Process for Foo {
        fn spawn() -> PendingHandle<Self> {
            Stage::spawn(|msg| async move {
                match msg {
                    Self::A => {
                        println!("A!");
                        Exit::Continue
                    },
                    Self::B => {
                        println!("B!");
                        Exit::Continue
                    },
                    Self::Stop => {
                        println!("Stop!");
                        Exit::Normal
                    },
                }
            })
        }
    }
    
    #[tokio::test]
    async fn example() {
        println!("test start");
        let pid = Foo::spawn().pid().await;
        println!("received pid");

        pid.send(Foo::A).await;
        println!("sent A");
        pid.send(Foo::B).await;
        println!("sent B");        
    }
}

// Implementation

pub trait Process: Sized {
    fn spawn() -> PendingHandle<Self>;
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Exit {
    Continue,       // Continue to run
    Normal,         // Exit normally, linked processes continue to run
    Error,          // Exit with error condition, terminate linked processes
}

/// Messages handled internally for all processes
pub enum SystemMsg {
    Link(UntypedPid),
    Unlink(UntypedPid),
    LinkExit(Exit),
}

/// Messages handled by the process loop inside of spawn
pub enum ProcessMsg<P> {
    System(SystemMsg),      // system implemented message
    Process(P),             // process implemented message
}

pub struct Stage;

impl Stage {
    pub fn spawn<P: Process, F, Fut>(mut f: F) -> PendingHandle<P>
    where
        P: Send + 'static,
        F: FnMut(P) -> Fut + Send + 'static,
        Fut: Future<Output=Exit> + Send + 'static
    {
        let (pid_sender, pid_receiver) = tokio::sync::oneshot::channel::<Pid<P>>();

        tokio::spawn(async move {
            println!("running async block");

            // Create a pid with process and system message senders
            let (sender, mut receiver) = tokio::sync::mpsc::channel::<ProcessMsg<P>>(1);
            let (sys_sender, mut sys_receiver) = tokio::sync::mpsc::channel::<SystemMsg>(1);
            let pid = Pid{ sender, sys_sender };

            // Send the pid back to the PendingPid so it can be upgraded to a Pid
            pid_sender.send(pid).ok();
            println!("sent pid");

            // Processes linked to receive exit messages
            let mut links: Vec<UntypedPid> = Vec::new();
            let mut exit = Exit::Continue;

            // Process incoming ProcessMsg, exiting the loop if the process definition
            // function returns an exit, or if a linked process exits abnormally
            loop {
                println!("waiting for msg");

                let msg = tokio::select! {
                    Some(msg) = receiver.recv() => msg,
                    Some(msg) = sys_receiver.recv() => ProcessMsg::System(msg),
                };

                println!("received msg");
                match msg {
                    ProcessMsg::Process(process_msg) => {
                        // Delegate processing to process function
                        let exit = f(process_msg).await;

                        // Exit if the process requests
                        if exit != Exit::Continue {
                            break;
                        }        
                    },
                    ProcessMsg::System(system_msg) => {
                        match system_msg {
                            // Link a process
                            SystemMsg::Link(pid) => {
                                links.push(pid);
                            },

                            // Unlink a process
                            SystemMsg::Unlink(pid) => {
                                // TODO: Fix this... unable to do the following because tokio::mpsc::Sender
                                // does not implement PartialEq.
                                // links.retain(|&x| x != pid);

                            },

                            // Exit if linked process exits abnormally
                            SystemMsg::LinkExit(linked_exit) => {
                                exit = linked_exit;
                                break;
                            },
                        }
                    },
                }
            }
            println!("loop finished");

            if exit != Exit::Normal {
                for link in links.iter() {
                    link.send(SystemMsg::LinkExit(exit)).await;
                }
            }

            println!("async block exiting");
        });

        // Send PendingPid to the caller
        PendingHandle{ receiver: pid_receiver }
    }
}

pub struct PendingHandle<P: Process> {
    receiver: tokio::sync::oneshot::Receiver<Pid<P>>,
}

impl<P: Process> PendingHandle<P> {
    pub async fn pid(self) -> Pid<P> {
        self.receiver.await.unwrap()
    }
}

pub struct Pid<P: Process> {
    sender: tokio::sync::mpsc::Sender<ProcessMsg<P>>,
    sys_sender: tokio::sync::mpsc::Sender<SystemMsg>,
}

impl<P: Process> Pid<P> {
    pub async fn send(&self, msg: P) {
        self.sender.send(ProcessMsg::Process(msg)).await.ok();
    }

    fn untyped(&self) -> UntypedPid {
        UntypedPid { sender: self.sys_sender.clone() }
    }
}

/// A Pid which can only send system messages
#[derive(Clone)]
pub struct UntypedPid {
    sender: tokio::sync::mpsc::Sender<SystemMsg>,
}

impl UntypedPid {
    pub async fn send(&self, msg: SystemMsg) {
        self.sender.send(msg).await.ok();
    }

    pub async fn link(&self, other_pid: &UntypedPid) {
        let (_self_link, _other_link) = tokio::join! {
            self.send(SystemMsg::Link(other_pid.clone())),
            other_pid.send(SystemMsg::Link(self.clone())),
        };
    }
}