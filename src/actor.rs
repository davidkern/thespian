use std::fmt::Debug;

use crate::link;

/// `Actor` maintains a connection to its `Process` to allow
/// `Actor` methods to be implemented via functions sent to
/// the `Process`.
pub struct Actor<State, Reply> {
    sender: link::Sender<State, Reply>
}

impl<State, Reply> Clone for Actor<State, Reply> {
    /// `Actor` may be freely cloned and share the backing `Process`.
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

impl<State, Reply> Actor<State, Reply>
    where
        State: Debug,
{
    pub(crate) fn new_with_sender(sender: link::Sender<State, Reply>) -> Self {
        Self {
            sender,
        }
    }

    /// Executes a function in the `Process` with a reference to the state.
    pub fn call_ref(&self, caller: fn(&State)) {
        self.sender.send(link::Message::Ref(caller)).ok();
    }

    /// Executes a function in the `Process` with a mutable reference to the state.
    pub fn call_ref_mut(&self, caller: fn(&mut State)) {
        self.sender.send(link::Message::RefMut(caller)).ok();
    }

    /// Executes a function in the `Process` with a reference to the state and a `ReplySender`.
    pub async fn call_ref_reply(&self, caller: fn(&State, link::ReplySender<Reply>)) -> Reply {
        let (reply_sender, reply_receiver) = link::new_reply();
        self.sender.send(link::Message::RefReply(caller, reply_sender)).ok();

        reply_receiver.recv().await.ok().unwrap()
    }

    /// Executes a function in the `Process` with a mutable reference to the state and a `ReplySender`.
    pub async fn call_ref_mut_reply(&self, caller: fn(&State, link::ReplySender<Reply>)) -> Reply {
        let (reply_sender, reply_receiver) = link::new_reply();
        self.sender.send(link::Message::RefMutReply(caller, reply_sender)).ok();

        reply_receiver.recv().await.ok().unwrap()
    }
}
