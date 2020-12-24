use std::future::Future;
use futures::stream;
use futures::StreamExt;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::marker::PhantomData;
use tokio::sync::oneshot;
use crate::link;

pub struct Pid<Msg> {
    sender: link::Sender<Msg>,
}

impl<Msg> Pid<Msg> {
    pub async fn send(&self, msg: Msg) {
        self.sender.send(msg).await;
    }
}

pub struct Process<F, Fut, Msg>
where
    F: FnMut(Msg) -> Fut,
    Fut: Future<Output=()>
{
    f: F,
    receiver: link::Receiver<Msg>,
    _fut: PhantomData<Fut>,
    _msg: PhantomData<Msg>,
}

impl<F, Fut, Msg> Process<F, Fut, Msg>
where
    F: FnMut(Msg) -> Fut,
    Fut: Future<Output=()>
{
    pub fn new(f: F) -> (Self, Pid<Msg>) {
        let (sender, receiver) = link::Link::new().split();
        let pid = Pid{ sender };

        let process = Self {
            f,
            receiver,
            _fut: PhantomData,
            _msg: PhantomData,
        };

        (process, pid)
    }

    pub async fn start(&mut self)
    {
        while let Some(input) = self.receiver.recv().await {
            (self.f)(input).await
        }
    }
}

type BoxedProcessFuture = Box<dyn Future<Output=()>>;

pub struct Stage {
    link: link::Link<BoxedProcessFuture>,
}

impl Stage {
    pub fn new() -> Self {
        Self {
            link: Default::default(),
        }
    }

    pub async fn start(&mut self)
    {
        
    }

    pub async fn spawn<Msg, F: FnMut(Msg) -> Fut, Fut: Future<Output=()>>(&mut self, f: F) -> Pid<Msg>
    {
        let (mut process, pid) = Process::new(f);
        // let process_fut = Box::new(process.start());
        // self.link.send(process_fut).await;

        pid
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn start_a_process() {
        let (mut process, pid) = Process::new(|msg: &'static str| async move {
            println!("process {}...", msg);
        });

        tokio::join! {
            process.start(),
            async move {
                pid.send("first").await;
                pid.send("second").await;
                pid.send("third").await;
            }
        };
    }

    #[tokio::test]
    async fn with_a_stage() {
        let mut stage = Stage::new();

        let pid = stage.spawn(|msg: &'static str| async move {
            println!("process {}...", msg);
        }).await;

        tokio::join! {
            stage.start(),
            async move {
                pid.send("first").await;
                pid.send("second").await;
                pid.send("third").await;
            }
        };
    }
}
