use std::fmt::Debug;

use crate::actor::Actor;
use crate::link;

/// `Process` holds the `Actor` state and sequentially processes
/// calls sent from the `Actor`.
pub struct Process<State, Reply>
{
    state: State,
    receiver: link::Receiver<State, Reply>,
}

impl<State, Reply> Process<State, Reply>
    where
        State: Debug,
{
    /// Creates a new (`Process`, `Actor`) pair given an initial state.
    pub fn new_with_state(state: State) -> (Self, Actor<State, Reply>) {
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
