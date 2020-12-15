use crate::actor::Actor;
use crate::link;
use crate::link::Link;

pub trait Process {
    type State;
    type Message;
    type Receiver;
}

pub struct ProcessImpl<State, Receiver>
where
    Receiver: link::LinkReceiver,
{
    state: State,
    receiver: Receiver,
}

pub enum VisitorMessage<State, Reply> {
    Ref(fn(&State)),
    RefMut(fn(&mut State)),
    RefReply(fn(&State, link::ReplySender<Reply>), link::ReplySender<Reply>),
    RefMutReply(fn(&State, link::ReplySender<Reply>), link::ReplySender<Reply>),
}

pub type VisitorProcess<State, Reply> = ProcessImpl<State, link::UnboundedReceiver<VisitorMessage<State, Reply>>>;

impl<State, Reply> Process for VisitorProcess<State, Reply> {
    type State = State;
    type Message = VisitorMessage<State, Reply>;
    type Receiver = link::UnboundedReceiver<Self::Message>;
}

impl<State, Reply> VisitorProcess<State, Reply>
{
    /// Creates a new (`Process`, `Actor`) pair given an initial state.
    pub fn new_with_state(state: State) -> (Self, Actor<State, Reply>) {
        let (sender, receiver) = link::UnboundedLink::new();
        (
            Self {
                state,
                receiver,
            },
            Actor::from(sender)
        )
    }

    /// Starts the `Process`.  The corresponding `Actor` is alive as long
    /// as the `Process` is running.
    pub async fn start(&mut self) {
        while let Some(call) = self.receiver.recv().await {
            match call {
                VisitorMessage::Ref(caller) => {
                    caller(&self.state);
                },
                VisitorMessage::RefMut(caller) => {
                    caller(&mut self.state);
                },
                VisitorMessage::RefReply(caller, reply_sender) => {
                    caller(&self.state, reply_sender);
                },
                VisitorMessage::RefMutReply(caller, reply_sender) => {
                    caller(&mut self.state, reply_sender);
                }
            }
        }
    }
}
