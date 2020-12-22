use std::future::Future;

/// All processes are spawned in an `Arena`, which owns the
/// process state and messages between processes.  When an arena is
/// dropped, all processes are also dropped.
/// 
/// Multiple arenas may exist in an application - processes in
/// different scopes are entirely isolated from one another. Message passing
/// between processes in linked scopes may be added in the future.
pub struct Arena;

impl Arena {
    /// Spawn a process by passing self into spawner
    pub fn spawn(&mut self, spawner: fn(&mut Self) -> Pid) -> Pid {
        spawner(self)
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
pub trait ProcessDef {
    type Input;
    type Output;

    fn spawn(arena: &mut Arena) -> Pid;
}

pub struct Process<P: ProcessDef>(P);

/// A `Process` consumes a Self::Input type of input, asynchronously processes
/// that input, and returns a Self::Output type of output.
impl<P: ProcessDef> Process<P> {
    pub fn new(fut: impl Future<Output=Result<P::Output, ExitReason>>) {

    }

    pub fn spawn<Arena>(arena: Arena) {

    }

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

    struct Toggle(bool);

    impl ProcessDef for Toggle {
        type Input = bool;
        type Output = ();

        fn spawn(arena: Arena) -> Pid {
            arena.spawn()
        }
    }

    #[tokio::test]
    async fn start_a_process() {

    }
}
