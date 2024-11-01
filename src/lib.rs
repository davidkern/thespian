//! Goal:
//!   * An actor system
//!   * An actor's messages are defined by traits
//!   * A (trait object, address) is a capability to send messages to an actor


// Strategy:
//  * Figure out the desugared representation
//  * Implement runtime
//  * Proc macro for sugared representation


// Sugared representation

//#[capability]
trait Memory {
    fn get(&self) -> Option<usize>;
    fn set(&mut self, value: Option<usize>);
}

//#[thespian(Memory)]
#[derive(Default)]
pub struct Actor {
    value: Option<usize>,
}

impl Memory for Actor {
    fn set(&mut self, value: Option<usize>) {
        self.value = value;
    }

    fn get(&self) -> Option<usize> {
        self.value
    }
}

// Desugared representation

const _: () = {
    use crate::runtime::{Thespian, Runtime};
    use std::sync::mpsc;

    pub enum Message {
        Set(Option<usize>),
        Get(mpsc::Sender<Option<usize>>),
    }

    pub struct Capability {
        input: mpsc::Sender<Message>,
    }

    impl Thespian for Actor {
        type Capability = Capability;

        fn spawn(self, _rt: Runtime) -> Self::Capability {
            let (tx, rx) = mpsc::channel();

            std::thread::spawn(move || {
                run(self, rx)
            });

            Capability {
                input: tx,
            }
        }
    }

    impl Memory for Capability {
        fn get(&self) -> Option<usize> {
            let (tx, rx) = mpsc::channel();
            let _ = self.input.send(Message::Get(tx));

            // TODO: implement recovery to deal with errors
            rx.recv().unwrap()
        }

        fn set(&mut self, value: Option<usize>) {
            // TODO: implement recovery to deal with errors
            let _ = self.input.send(Message::Set(value));
        }
    }

    // TODO: allow tuple of receivers so actor can implement multiple capabilities
    fn run(mut this: Actor, rx: mpsc::Receiver<Message>) {
        while let Ok(msg) = rx.recv() {
            match msg {
                Message::Set(value) => this.set(value),
                Message::Get(ret) => {
                    let _ = ret.send(this.get());
                },
            }
        }
    }
};

// Runtime
pub mod runtime {
    pub struct Runtime;

    pub trait Thespian: Sized {
        type Capability;

        fn spawn(self, rt: Runtime) -> Self::Capability;
    }

    pub struct Return<T>(std::marker::PhantomData<T>);

    impl<T> Return<T> {
        pub fn send(&self, value: T) {
            todo!();
        }
    }        
}

#[cfg(test)]
mod test {
    use runtime::{Runtime, Thespian};
    use super::*;

    #[test]
    fn sync_usage() {
        let rt = Runtime;
        let mut memory = Actor::default().spawn(rt);

        assert_eq!(memory.get(), None);
        memory.set(Some(1));
        assert_eq!(memory.get(), Some(1));
    }
}
