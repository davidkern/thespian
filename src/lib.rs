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
//! use thespian::{Actor, VisitorProcess};
//!
//! #[tokio::main]
//! pub async fn main() {
//!     let (mut process, actor) =
//!         VisitorProcess::<State, State>::new_with_state(State::Alpha);
//!     let toggle = Toggle::from(actor);
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
//! struct Toggle(Actor<State, State>);
//!
//! impl From<Actor<State, State>> for Toggle {
//!     fn from(actor: Actor<State, State>) -> Self {
//!         Self(actor)
//!     }
//! }
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

mod actor;
mod link;
mod process;
mod scratchpad;

pub use actor::Actor;
pub use process::VisitorProcess;
