//! Link between concurrent tasks

use tokio::sync::mpsc;

// Sender

pub struct Sender<T>(mpsc::Sender<T>);

impl<T> Sender<T> {
    pub async fn send(&self, value: T) {
        match self.0.send(value).await {
            Ok(()) => {},
            Err(_) => {},
        };
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// Receiver

pub struct Receiver<T>(mpsc::Receiver<T>);

impl<T> Receiver<T> {
    pub async fn recv(&mut self) -> Option<T> {
        self.0.recv().await
    }
}
