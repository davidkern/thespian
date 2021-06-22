//! Finite State Machine
use concurrent_queue::ConcurrentQueue;
use std::fmt::Debug;


pub struct MatchCell<T>
where
    T: Debug
{
    q: ConcurrentQueue<T>,
}

impl<T: Debug> MatchCell<T> {
    pub fn new(inner: T) -> Self {
        let q = ConcurrentQueue::bounded(1);
        q.push(inner).unwrap();

        Self { q }
    }

    pub async fn when<F>(&self, f: F)
    where
        F: Fn(*mut T)
    {

    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug)]
    enum State {
        A,
        B,
        C,
    }

    #[test]
    fn example() {
        let cell = MatchCell::new(State::A);
    }
}
