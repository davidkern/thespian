use tokio::sync::{
    mpsc::{
        UnboundedSender,
        UnboundedReceiver,
    },
    oneshot,
};
use tokio::sync::mpsc::unbounded_channel;

pub fn new<State, Reply>() -> (Sender<State, Reply>, Receiver<State, Reply>) {
    let (sender, receiver) = unbounded_channel();
    (sender, Receiver(receiver))
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

pub type Sender<State, Reply> = UnboundedSender<Message<State, Reply>>;
pub type ReplySender<Reply> = oneshot::Sender<Reply>;
