//! Link between concurrent tasks

use tokio::sync::mpsc;

pub struct Link<T>(Sender<T>, Receiver<T>);

// Link

impl<T> Link<T> {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(1);
        Self(Sender(tx), Receiver(rx))
    }

    pub fn split(self) -> (Sender<T>, Receiver<T>) {
        (self.0, self.1)
    }
}

// Sender

pub struct Sender<T>(mpsc::Sender<T>);

impl<T> From<mpsc::Sender<T>> for Sender<T> {
    fn from(sender: mpsc::Sender<T>) -> Self {
        Self(sender)
    }
}

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

impl<T> From<mpsc::Receiver<T>> for Receiver<T> {
    fn from(receiver: mpsc::Receiver<T>) -> Self {
        Self(receiver)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(PartialEq, Debug)]
    enum Msg {
        A, B
    }

    #[tokio::test]
    async fn send_and_receive() {
        let (sender, mut receiver) = Link::new().split();

        let (_sent, _received) = tokio::join! {
            // send Msg::A from sender and Msg::B from a clone
            async {
                sender.send(Msg::A).await;
                sender.clone().send(Msg::B).await;
            },

            async {
                assert_eq!(Msg::A, receiver.recv().await.unwrap());
                assert_eq!(Msg::B, receiver.recv().await.unwrap());
            }
        };
    }
}
