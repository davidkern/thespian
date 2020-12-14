use tokio::sync::{
    mpsc::{
        UnboundedSender,
        UnboundedReceiver,
    },
    oneshot,
};

pub(crate) enum Call<State, Reply> {
    Ref(fn(&State)),
    RefMut(fn(&mut State)),
    RefReply(fn(&State, ReplySender<Reply>), ReplySender<Reply>),
    RefMutReply(fn(&State, ReplySender<Reply>), ReplySender<Reply>),
}

pub(crate) type CallReceiver<State, Reply> = UnboundedReceiver<Call<State, Reply>>;
pub(crate) type CallSender<State, Reply> = UnboundedSender<Call<State, Reply>>;
pub(crate) type ReplySender<Reply> = oneshot::Sender<Reply>;
