//! # Redesign
//!
//! GOAL: Simple supervision tree
//! 
//! TODO: Documentation

use std::time::{SystemTime, UNIX_EPOCH};

pub fn log(msg: &str) {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("[{}] {}", timestamp.as_millis(), msg);
}

pub struct Running { }

impl Running {
    fn new() -> Running {
        Running {
        }
    }

    /// Waits for the future to complete and returns an Exited
    pub async fn run<Fut>(self, fut: Fut) -> Exited
    where
        Fut: std::future::Future<Output=Reason> + Send + 'static
    {
        log("awaiting future");
        let exited = Exited::new(fut.await);
        log("future complete");
        exited
    }
}

impl Drop for Running {
    fn drop(&mut self) {
        log("dropped");
    }
}

pub enum Reason {
    Normal,
    Abnormal,
}

pub struct Exited {
    reason: Reason,
}

impl Exited {
    fn new(reason: Reason) -> Exited {
        Exited {
            reason,
        }
    }
}

pub struct Actor {
}

impl Actor {
    fn spawn<ActorFn, Fut>(actor_fn: ActorFn) -> Actor
    where
        ActorFn: Fn(Running) -> Fut,
        Fut: std::future::Future<Output=Exited> + Send + 'static
    {
        let running = Running::new();
        let fut = actor_fn(running);
        smol::spawn(fut).detach();    

        Actor {
            //fut: Box::new(actor_fn(context)),
        }
    }
}

pub struct Spec {
    spawn_fn: Box<dyn Fn()>,
}

impl Spec {
    /// Creates a new Spec, which spawns actors from the ActorFn.
    pub fn new<ActorFn, Fut>(actor_fn: ActorFn) -> Spec
    where
        ActorFn: Fn(Running) -> Fut + 'static,
        Fut: std::future::Future<Output=Exited> + Send + 'static,
    {
        Spec {
            spawn_fn: Box::new(move || {
                Actor::spawn(&actor_fn);
            }),
        }
    }

    /// Spawns an Actor defined by this Spec
    fn spawn(&self) {
        (self.spawn_fn)();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use smol::Timer;
    use std::time::Duration;

    #[test]
    fn spawn_actor() {
        log("starting");

        let spec = Spec::new(|running| async move {
            running.run(async {
                log("spawned a thing");
                Timer::after(Duration::from_millis(10)).await;

                Reason::Normal
            }).await
        });

        log("about to spawn");
        spec.spawn();
        spec.spawn();

        log("done spawning");

        smol::block_on(async {
            log("starting to wait");
            Timer::after(Duration::from_secs(1)).await;
        });

        log("done");
    }
}
