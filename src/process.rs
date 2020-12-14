use crate::actor::Actor;
use crate::link;

pub struct ProcessState<State, Receiver> {
    state: State,
    receiver: Receiver,
}

pub trait Process {
    type State;
    type Receiver;
}

pub type VisitorProcess<State, Message> = ProcessState<State, link::Receiver<State, Message>>;

impl<State, Message> VisitorProcess<State, Message>
{
    /// Creates a new (`Process`, `Actor`) pair given an initial state.
    pub fn new_with_state(state: State) -> (Self, Actor<State, Message>) {
        let (sender, receiver) = link::new();
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
                link::Message::Ref(caller) => {
                    caller(&self.state);
                },
                link::Message::RefMut(caller) => {
                    caller(&mut self.state);
                },
                link::Message::RefReply(caller, reply_sender) => {
                    caller(&self.state, reply_sender);
                },
                link::Message::RefMutReply(caller, reply_sender) => {
                    caller(&mut self.state, reply_sender);
                }
            }
        }
    }
}
