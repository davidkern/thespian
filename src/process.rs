use std::future::Future;
use futures::stream;
use futures::StreamExt;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::marker::PhantomData;
use crate::link;


pub struct Pid<Msg> {
    sender: link::Sender<Msg>,
}

impl<Msg> Pid<Msg> {
    pub async fn send(&self, msg: Msg) {
        self.sender.send(msg);
    }
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
    pub fn new(f: F) -> Self {
        Self {
            f,
            _fut: PhantomData,
            _msg: PhantomData,
        }
    }

    pub async fn start(&mut self)
    {
        let (sender, mut receiver) = link::Link::new().split();

        let pid = Pid{ sender };

        while let Some(input) = receiver.recv().await {
            (self.f)(input).await
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Summation {
        value: u64,
    }

    #[tokio::test]
    async fn start_a_process() {
        let mut process = Process::new(|msg: ()| async move {

        });

        process.start();
    }
}
