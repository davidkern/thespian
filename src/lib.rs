//! # Introduction
//!
//! Thespian implements a basic actor system in async Rust inspired
//! heavily by Erlang Processes and Supervisors.
//!
//! Instead of providing a direct messaging interface to the `Actor`,
//! methods defined by the package user on `impl Actor<State, Reply>`
//! are used as the external API for the actor.  This makes usage of
//! an `Actor` no different than using a reference to a struct.
//!
//! Internal to the `Actor` implementation a mechanism is provided to
//! run a function inside the `Process` holding the `Actor` state.
//! Since all access to the state is handled serially by the `Process`,
//! the behavior of the system resembles a typical `Actor` system.
//!
//! While technically the `Process` would be called the actor in typical
//! usage of the term, for this package the actor's mailbox is instead
//! called `Actor` for better ergonomics.  Defining methods on a mailbox
//! felt odd and confusing.
//!
//! ## TODO
//!
//! * Fix ergonomics around use of newtype pattern for Actor
//! * Implement supervision tree
//!
//! ## Example usage
//!
//! ```
//! #[tokio::main]
//! pub async fn main() {
//!     let (mut process, actor) =
//!         thespian::Process::<State, State>::new_with_state(State::Alpha);
//!     let toggle = Toggle(actor);
//!
//!     let (_process_result, _task_result) = tokio::join! {
//!         async move {
//!             process.start().await;
//!         },
//!         async move {
//!             toggle.flip();
//!             toggle.flip();
//!             println!("get: {:?}", toggle.get().await);
//!         }
//!     };
//! }
//!
//! #[derive(Copy, Clone, Debug)]
//! enum State {
//!     Alpha,
//!     Beta,
//! }
//!
//! struct Toggle(thespian::Actor<State, State>);
//!
//! impl Toggle {
//!     pub async fn get(&self) -> State {
//!         self.0.call_ref_reply(|state, reply| {
//!             reply.send(state.clone());
//!         }).await
//!     }
//!
//!     pub fn flip(&self) {
//!         self.0.call_ref_mut(|state| {
//!             println!("state: {:?}", state);
//!             match state {
//!                 State::Alpha => *state = State::Beta,
//!                 State::Beta => *state = State::Alpha,
//!             }
//!         });
//!     }
//! }
//! ```

use tokio::sync::{
    mpsc::{
        unbounded_channel,
        UnboundedSender,
        UnboundedReceiver,
    },
    oneshot,
};
use std::fmt::Debug;

enum Call<State, Reply> {
    Ref(fn(&State)),
    RefMut(fn(&mut State)),
    RefReply(fn(&State, ReplySender<Reply>), ReplySender<Reply>),
    RefMutReply(fn(&State, ReplySender<Reply>), ReplySender<Reply>),
}

/// `Process` holds the `Actor` state and sequentially processes
/// calls sent from the `Actor`.
pub struct Process<State, Reply>
{
    state: State,
    receiver: CallReceiver<State, Reply>,
}

/// `Actor` maintains a connection to its `Process` to allow
/// `Actor` methods to be implemented via functions sent to
/// the `Process`.
pub struct Actor<State, Reply> {
    sender: CallSender<State, Reply>
}

impl<State, Reply> Clone for Actor<State, Reply> {
    /// `Actor` may be freely cloned and share the backing `Process`.
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

type CallReceiver<State, Reply> = UnboundedReceiver<Call<State, Reply>>;
type CallSender<State, Reply> = UnboundedSender<Call<State, Reply>>;
type ReplySender<Reply> = oneshot::Sender<Reply>;

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

impl<State, Reply> Actor<State, Reply>
    where
        State: Debug,
{
    fn new_with_sender(sender: CallSender<State, Reply>) -> Self {
        Self {
            sender,
        }
    }

    /// Executes a function in the `Process` with a reference to the state.
    pub fn call_ref(&self, caller: fn(&State)) {
        self.sender.send(Call::Ref(caller)).ok();
    }

    /// Executes a function in the `Process` with a mutable reference to the state.
    pub fn call_ref_mut(&self, caller: fn(&mut State)) {
        self.sender.send(Call::RefMut(caller)).ok();
    }

    /// Executes a function in the `Process` with a reference to the state and a `ReplySender`.
    pub async fn call_ref_reply(&self, caller: fn(&State, ReplySender<Reply>)) -> Reply {
        let (reply_sender, reply_receiver) = oneshot::channel();
        self.sender.send(Call::RefReply(caller, reply_sender)).ok();

        reply_receiver.await.ok().unwrap()
    }

    /// Executes a function in the `Process` with a mutable reference to the state and a `ReplySender`.
    pub async fn call_ref_mut_reply(&self, caller: fn(&State, ReplySender<Reply>)) -> Reply {
        let (reply_sender, reply_receiver) = oneshot::channel();
        self.sender.send(Call::RefMutReply(caller, reply_sender)).ok();

        reply_receiver.await.ok().unwrap()
    }
}
