use tokio::sync::{
    mpsc::{
        UnboundedSender,
        UnboundedReceiver,
    },
    oneshot,
};
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::error::SendError;

pub fn new<State, Reply>() -> (Sender<State, Reply>, Receiver<State, Reply>) {
    let (sender, receiver) = unbounded_channel();
    (Sender(sender), Receiver(receiver))
}

pub enum Message<State, Reply> {
    Ref(fn(&State)),
    RefMut(fn(&mut State)),
    RefReply(fn(&State, ReplySender<Reply>), ReplySender<Reply>),
    RefMutReply(fn(&State, ReplySender<Reply>), ReplySender<Reply>),
}

pub struct Receiver<State, Reply>(UnboundedReceiver<Message<State, Reply>>);

impl<State, Reply> Receiver<State, Reply> {
    pub async fn recv(&mut self) -> Option<Message<State, Reply>> {
        self.0.recv().await
    }
}

pub struct Sender<State, Reply>(UnboundedSender<Message<State, Reply>>);

impl<State, Reply> Sender<State, Reply> {
    pub fn send(&self, msg: Message<State, Reply>) -> Result<(), SendError<Message<State, Reply>>> {
        self.0.send(msg)
    }
}

impl<State, Reply> Clone for Sender<State, Reply> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub type ReplySender<Reply> = oneshot::Sender<Reply>;
