use std::future::Future;
use std::marker::Send;

#[cfg(test)]
mod test {
    use super::*;

    pub enum Foo {
        A,
        B,
    }
    
    impl Process for Foo {
        fn spawn() -> PendingPid<Self> {
            Stage::spawn(|msg: Self| async move {
                match msg {
                    Self::A => {
                        println!("A!");
                    },
                    Self::B => {
                        println!("B!");
                    }
                }
            })
        }
    }
    
    #[tokio::test]
    async fn example() {
        let pid = Foo::spawn().pid().await;
        pid.send(Foo::A).await;
        pid.send(Foo::B).await;

        // This send will never be processed, but forces the previous
        // message to be processed.  Program exits before the process
        // receives this one, and can't fix that until there is a way
        // to await process exit.
        pid.send(Foo::B).await;
    }    
}

// Implementation

pub trait Process: Sized {
    fn spawn() -> PendingPid<Self>;
}

pub struct Stage;

impl Stage {
    pub fn spawn<P: Process, F, Fut>(mut f: F) -> PendingPid<P>
    where
        P: Send + 'static,
        F: FnMut(P) -> Fut + Send + 'static,
        Fut: Future<Output=()> + Send + 'static
    {
        let (pid_sender, pid_receiver) = tokio::sync::oneshot::channel::<Pid<P>>();

        tokio::spawn(async move {
            let (sender, mut receiver) = tokio::sync::mpsc::channel::<P>(1);
            pid_sender.send(Pid{ sender }).ok();

            while let Some(msg) = receiver.recv().await {
                f(msg).await;
            }

            println!("exiting");
        });

        PendingPid{ receiver: pid_receiver }
    }
}

pub struct PendingPid<P: Process> {
    receiver: tokio::sync::oneshot::Receiver<Pid<P>>,
}

impl<P: Process> PendingPid<P> {
    pub async fn pid(self) -> Pid<P> {
        self.receiver.await.unwrap()
    }
}

pub struct Pid<P: Process> {
    sender: tokio::sync::mpsc::Sender<P>,
}

impl<P: Process> Pid<P> {
    pub async fn send(&self, msg: P) {
        self.sender.send(msg).await.ok();
    }
}
