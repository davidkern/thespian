use tokio::sync::{
    mpsc::{
        unbounded_channel,
    },
};
use std::fmt::Debug;

use crate::actor::Actor;
use crate::link::{Call, CallReceiver};

/// `Process` holds the `Actor` state and sequentially processes
/// calls sent from the `Actor`.
pub struct Process<State, Reply>
{
    state: State,
    receiver: CallReceiver<State, Reply>,
}

impl<State, Reply> Process<State, Reply>
    where
        State: Debug,
{
    /// Creates a new (`Process`, `Actor`) pair given an initial state.
    pub fn new_with_state(state: State) -> (Self, Actor<State, Reply>) {
        let (sender, receiver) = unbounded_channel();
        (
            Self {
                state,
                receiver,
            },
            Actor::new_with_sender(sender)
        )
    }

    /// Starts the `Process`.  The corresponding `Actor` is alive as long
    /// as the `Process` is running.
    pub async fn start(&mut self) {
        while let Some(call) = self.receiver.recv().await {
            match call {
                Call::Ref(caller) => {
                    caller(&self.state);
                },
                Call::RefMut(caller) => {
                    caller(&mut self.state);
                },
                Call::RefReply(caller, reply_sender) => {
                    caller(&self.state, reply_sender);
                },
                Call::RefMutReply(caller, reply_sender) => {
                    caller(&mut self.state, reply_sender);
                }
            }
        }
    }
}
