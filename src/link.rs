use tokio::sync::{mpsc, oneshot};
use tokio::sync::mpsc::unbounded_channel;
use std::marker::PhantomData;

pub trait Link {
    type Message;
    type Sender: LinkSender<Message=Self::Message>;
    type Receiver: LinkReceiver<Message=Self::Message>;

    fn new() -> (Self::Sender, Self::Receiver);
}

pub trait LinkReceiver {
    type Message;
}

pub trait LinkSender {
    type Message;
}

pub struct UnboundedLink<Msg> {
    _msg: PhantomData<Msg>,
}

impl<Msg> Link for UnboundedLink<Msg> {
    type Message = Msg;
    type Sender = UnboundedSender<Msg>;
    type Receiver = UnboundedReceiver<Msg>;

    fn new() -> (Self::Sender, Self::Receiver) {
        let (sender, receiver) = unbounded_channel();
        (UnboundedSender(sender), UnboundedReceiver(receiver))
    }
}

pub struct UnboundedReceiver<Msg>(mpsc::UnboundedReceiver<Msg>);

impl<Msg> LinkReceiver for UnboundedReceiver<Msg> {
    type Message = Msg;
}

impl<Msg> UnboundedReceiver<Msg> {
    pub async fn recv(&mut self) -> Option<Msg> {
        self.0.recv().await
    }
}

pub struct UnboundedSender<Msg>(mpsc::UnboundedSender<Msg>);

impl<Msg> UnboundedSender<Msg> {
    pub fn send(&self, msg: Msg) -> Result<(), SendError<Msg>> {
        match self.0.send(msg) {
            Ok(x) => Ok(x),
            Err(e) => Err(SendError(e))
        }
    }
}

impl<Msg> LinkSender for UnboundedSender<Msg> {
    type Message = Msg;
}

impl<Msg> Clone for UnboundedSender<Msg> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub fn new_reply<Reply>() -> (ReplySender<Reply>, ReplyReceiver<Reply>) {
    let (sender, receiver) = oneshot::channel();
    (ReplySender(sender), ReplyReceiver(receiver))
}

pub struct ReplySender<Reply>(oneshot::Sender<Reply>);

impl<Reply> ReplySender<Reply> {
    pub fn send(self, reply: Reply) -> Result<(), Reply> {
        self.0.send(reply)
    }
}

pub struct ReplyReceiver<Reply>(oneshot::Receiver<Reply>);

impl<Reply> ReplyReceiver<Reply> {
    pub async fn recv(self) -> Result<Reply, RecvError> {
        match self.0.await {
            Ok(reply) => Ok(reply),
            Err(e) => Err(RecvError(e)),
        }
    }
}

pub struct RecvError(oneshot::error::RecvError);

pub struct SendError<Msg>(mpsc::error::SendError<Msg>);
