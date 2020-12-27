use std::future::Future;
use std::marker::Send;


pub struct Pid<Msg> {
    sender: tokio::sync::mpsc::Sender<Msg>,
}

impl<Msg> Pid<Msg> {
    fn new(sender: tokio::sync::mpsc::Sender<Msg>) -> Self {
        Self {
            sender,
        }
    }
}

pub struct Stage {
}

pub trait Process<Msg: Send> {
    fn sync_spawn(stage: &mut Stage, sender: tokio::sync::oneshot::Sender<Pid<Msg>>);
}

impl Stage {
    pub async fn spawn<Msg: Send>(&mut self, process: impl Process<Msg>) -> Pid<Msg> {
        let (pid_sender, pid_receiver) = tokio::sync::oneshot::channel::<Pid<Msg>>();

        process.sync_spawn(self, pid_sender);
    
        pid_receiver.await.unwrap()
    }

    fn sync_spawn<F, Fut, Msg>(&mut self, pid_sender: tokio::sync::oneshot::Sender<Pid<Msg>>, mut f: F)
    where
        F: FnMut(Msg) -> Fut + Send + 'static,
        Fut: Future<Output=()> + Send + 'static,
        Fut::Output: Send,
        Msg: Send + 'static,
    {
        tokio::spawn(async move {
            let (tx, mut rx) = tokio::sync::mpsc::channel::<Msg>(1);
            pid_sender.send(Pid::new(tx)).ok();
            while let Some(msg) = rx.recv().await {
                f(msg).await
            }
        });
    }
}

// Example Usage

pub struct MyProcess;

impl Process<bool> for MyProcess {
    fn sync_spawn(stage: &mut Stage, sender: tokio::sync::oneshot::Sender<Pid<bool>>) {
        stage.sync_spawn(sender, |_msg: bool| async move {

        });
    }
}
