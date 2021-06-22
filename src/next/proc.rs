use std::future::Future;
use std::mem;
use std::panic::{self, AssertUnwindSafe};
use std::pin::Pin;
use std::sync::Arc;
use anyhow::{anyhow, Context, Result, Error};
use smol::channel::{bounded, Sender, Receiver};
use smol::lock::RwLock;

pub struct AsyncState<TState> {
    sender: Sender<TState>,
    receiver: Receiver<TState>,
}

impl<TState> AsyncState<TState> {
    async fn store(&self, state: TState) {
        self.sender.send(state).await.unwrap();
    }

    async fn load(&self) -> TState {
        self.receiver.recv().await.unwrap()
    }

    pub async fn map(&self, mut func: impl FnMut(TState) -> TState) -> &Self {
        let state = self.load().await;
        let next_state = (func)(state);
        self.store(next_state).await;

        self
    }
}

pub async fn async_state<TState>(state: TState) -> AsyncState<TState> {
    let (sender, receiver) = bounded(1);
    sender.send(state).await.unwrap();
    AsyncState { sender, receiver }
}

pub trait ProcDef {
    type Output: Send + 'static;

    fn proc(self) -> AsyncProc<Self::Output>;
}

pub struct AsyncProc<TOutput> {
    fut: Pin<Box<dyn Future<Output=Result<TOutput>> + Send + 'static>>
}

impl<TOutput> AsyncProc<TOutput> {
    pub fn new(fut: impl Future<Output=Result<TOutput>> + Send + 'static) -> Self {
        Self {
            fut: Box::pin(fut)
        }
    }
}

pub struct Proc<TProcDef>
where
    TProcDef: ProcDef
{
    //old_state: Arc<RwLock<State<TProcDef>>>,
    state: AsyncState<State<TProcDef>>,
}

impl<TProcDef: ProcDef + Default> Default for Proc<TProcDef> {
    fn default() -> Self {
        Proc::new(TProcDef::default())
    }
}

impl<TProcDef: ProcDef> Proc<TProcDef>
{
    /// Creates a new Proc from a ProcDef
    pub fn new(proc_def: TProcDef) -> Self {
        Self {
            state: smol::block_on(async_state(State::Init { proc_def })),
        }
    }

    /// Starts the Proc
    pub async fn start(&self) -> Result<()> {
        // no writers will panic, so this will always succeed
        //let mut locked_state = self.old_state.write().await;
        let mut result = Ok(());

        self.state.map(|state| {
            if let State::Init { proc_def } = state {
                let proc_fut = proc_def.proc();
                let task = smol::spawn(proc_fut.fut);
                State::Running { task }
            } else {
                result = Err(anyhow!("already run"));
                State::Done { result: Err(anyhow!("already run")) }
            }
        }).await;

        result
    }
}

pub enum State<TProcDef>
where
    TProcDef: ProcDef
{
    Init {
        proc_def: TProcDef,
    },
    Starting,
    Running {
        task: smol::Task<Result<TProcDef::Output>>,
    },
    Done {
        result: Result<TProcDef::Output>,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Default)]
    struct Alpha {
        x: u32,
    }

    impl ProcDef for Alpha {
        type Output = ();

        fn proc(mut self) -> AsyncProc<()> {
            AsyncProc::new(async move {
                self.x += 1;

                println!("x = {}", self.x);

                Ok(())
            })
        }
    }

    #[test]
    fn example() {
        smol::block_on(async {
            // a proc can be created
            let alpha: Proc<Alpha> = Proc::new(Alpha::default());
            let _alpha2: Proc<Alpha> = Proc::default();

            // and started
            assert!(alpha.start().await.is_ok());

            // but not started again
            assert!(alpha.start().await.is_err());
        });
    }
}
