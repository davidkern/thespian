use std::future::Future;
use crate::link::Link;

/// All processes are spawned in an `Arena`, which owns the
/// process state and messages between processes.  When an arena is
/// dropped, all processes are also dropped.
/// 
/// Multiple arenas may exist in an application - processes in
/// different scopes are entirely isolated from one another. Message passing
/// between processes in linked scopes may be added in the future.
#[derive(Default)]
pub struct Arena<P: ProcessDef> {
    link: Link<P>,
}

impl<P: ProcessDef> Arena<P> {
    pub async fn start() {

    }

    /// Spawn a process by delegating to ProcessDef::spawner
    pub fn spawn(&mut self, process: P) {
        self.link.send(process);
    }
}

/// `ProcessDef` (Process Definition) allows definition of associated
/// types/constants and functions required by all process implementors.
/// However, the trait can not contain the definition of the async
/// Process::run function since `impl Future` is not possible in a trait.
/// 
/// One workaround is to use the async-trait crate to allow the definition of
/// async functions on traits.  However, there is a performance cost to using that
/// method with a large collection of small processes as is intended in this crate.
/// 
/// TODO: Implement a version of this with async-trait and measure performance 
/// against this implementation to quantify the above assertion.
pub trait ProcessDef: Sized + Default {
    type Input;
    type Output;
}

pub struct Process<P: ProcessDef>(P);

/// A `Process` consumes a Self::Input type of input, asynchronously processes
/// that input, and returns a Self::Output type of output.
impl<P: ProcessDef> Process<P> {
    pub async fn run(input: P::Input, fut: impl Future<Output=Result<P::Output, ExitReason>>) -> Result<P::Output, ExitReason> {
        fut.await
    }
}

pub struct Pid;

pub enum ExitReason {
    // TODO: replace `Abnormal` with actual reasons
    Abnormal,
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Default)]
    struct Swap(bool);

    impl Swap {
        pub async fn spawn(arena: &mut Arena<Self>) {
            Process::spawn(arena, |arena| {

            })
        }

        pub async fn process(&mut self, input: bool) -> bool {
            let original = self.0;
            self.0 = input;
            original
        }
    }

    impl ProcessDef for Swap {
        type Input = bool;
        type Output = bool;
    }

    #[tokio::test]
    async fn start_a_process() {
        let mut arena: Arena<Swap> = Default::default();
        arena.spawn();
    }
}
