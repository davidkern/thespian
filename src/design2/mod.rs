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
    
    impl Actor for Foo {
        fn spawn() -> PendingPid<Self> {
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

pub trait Actor: Sized {
    fn spawn() -> PendingPid<Self>;
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Exit {
    Continue,       // Continue to run
    Normal,         // Exit normally, linked processes continue to run
    Error,          // Exit with error condition, terminate linked processes
}

pub enum Msg<P> {
    LinkExit(Exit),
    Actor(P),
}

pub struct Stage;

impl Stage {
    pub fn spawn<P: Actor, F, Fut>(mut f: F) -> PendingPid<P>
    where
        P: Send + 'static,
        F: FnMut(P) -> Fut + Send + 'static,
        Fut: Future<Output=Exit> + Send + 'static
    {
        let (pid_sender, pid_receiver) = tokio::sync::oneshot::channel::<Pid<P>>();

        tokio::spawn(async move {
            println!("running async block");

            // Create a pid with a message sender and a process-exit sender
            let (sender, mut receiver) = tokio::sync::mpsc::channel::<Msg<P>>(1);
            let (exit_sender, exit_receiver) = tokio::sync::oneshot::channel::<Exit>();
            pid_sender.send(Pid{ sender }).ok();

            println!("sent pid");

            while let Some(msg) = receiver.recv().await {
                println!("received msg");

                match msg {
                    Msg::Actor(actor_msg) => {
                        let exit = f(actor_msg).await;
                        if exit != Exit::Continue {
                            println!("sending exit");
                            exit_sender.send(exit).ok();
                            break;
                        }        
                    },
                    Msg::LinkExit(exit) => {
                        println!("received exit {:?} from link", exit);
                        if exit == Exit::Error {
                            println!("sending exit received from link");
                            exit_sender.send(exit).ok();
                            break;
                        } else {
                            println!("ignoring exit received from link");
                        }
                    },
                }
            }

            println!("async block exiting");
        });

        PendingPid{ receiver: pid_receiver }
    }
}

pub struct PendingPid<P: Actor> {
    receiver: tokio::sync::oneshot::Receiver<Pid<P>>,
}

impl<P: Actor> PendingPid<P> {
    pub async fn pid(self) -> Pid<P> {
        self.receiver.await.unwrap()
    }
}

pub struct Pid<P: Actor> {
    sender: tokio::sync::mpsc::Sender<Msg<P>>,
}

impl<P: Actor> Pid<P> {
    pub async fn send(&self, msg: P) {
        self.sender.send(Msg::Actor(msg)).await.ok();
    }
}
