#![allow(unused_imports)]
#![allow(unused_variables)]

use std::future::Future;
use futures::stream;
use futures::StreamExt;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::marker::PhantomData;
use tokio::sync::oneshot;
use super::link;

pub struct Pid<Msg> {
    sender: link::Sender<Msg>,
}

impl<Msg> Pid<Msg> {
    pub async fn send(&self, msg: Msg) {
        self.sender.send(msg).await;
    }
}

pub trait ProcessType {
    type F;
    type Fut;
    type Msg;
}

impl<F, Fut, Msg> ProcessType for Process<F, Fut, Msg>
where
    F: FnMut(Msg) -> Fut,
    Fut: Future<Output=()>
{
    type F = F;
    type Fut = Fut;
    type Msg = Msg;
}

pub struct Process<F, Fut, Msg>
where
    F: FnMut(Msg) -> Fut,
    Fut: Future<Output=()>
{
    f: F,
    _fut: PhantomData<Fut>,
    _msg: PhantomData<Msg>,
}

impl<F, Fut, Msg> Process<F, Fut, Msg>
where
    F: FnMut(Msg) -> Fut,
    Fut: Future<Output=()>
{
    pub async fn new(f: F) -> Self {
        let process = Self {
            f,
            _fut: PhantomData,
            _msg: PhantomData,
        };

        process
    }

    pub async fn start(&mut self, receiver: &mut link::Receiver::<Msg>)
    {
        while let Some(msg) = receiver.recv().await {
            (self.f)(msg).await
        }
    }
}

pub enum StageMsg {
    Spawn,
}

pub struct Stage {
    link: link::Link<StageMsg>,
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


//    pub async fn spawn<Msg, F: FnMut(Msg) -> Fut, Fut: Future<Output=()>>(&mut self, f: F) -> Pid<Msg>
    pub async fn spawn<P: ProcessType>(&mut self, f: P::F) -> Pid<P::Msg>
    {
        let (tx, rx) = oneshot::channel::<P::F>();
        self.link.send(StageMsg::Spawn).await;

        let link: link::Link<P::Msg> = Default::default();
        let (sender, _receiver) = link.split();

        // Would need to send the receiver to the loop side of the stage here

        Pid{ sender }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[tokio::test]
    // async fn start_a_process() {
    //     let (mut process, pid) = Process::new(|msg: &'static str| async move {
    //         println!("process {}...", msg);
    //     });

    //     tokio::join! {
    //         process.start(),
    //         async move {
    //             pid.send("first").await;
    //             pid.send("second").await;
    //             pid.send("third").await;
    //         }
    //     };
    // }

    // #[tokio::test]
    // async fn with_a_stage() {
    //     let mut stage = Stage::new();

    //     let pid = stage.spawn(|msg: &'static str| async move {
    //         println!("process {}...", msg);
    //     }).await;

    //     tokio::join! {
    //         stage.start(),
    //         async move {
    //             pid.send("first").await;
    //             pid.send("second").await;
    //             pid.send("third").await;
    //         }
    //     };
    // }
}
