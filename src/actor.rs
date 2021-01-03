pub struct Context { }

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
        Exited::new(fut.await)
    }
}

impl Drop for Running {
    fn drop(&mut self) {
    }
}

pub enum Reason {
    Normal,
    Fault,
}

pub struct Exited {
    _reason: Reason,
}

impl Exited {
    fn new(reason: Reason) -> Exited {
        Exited {
            _reason: reason,
        }
    }
}

pub struct Actor {
}

impl Actor {
    fn spawn<ActorFn, Fut>(actor_fn: ActorFn) -> Actor
    where
        ActorFn: Fn(Context) -> Fut,
        Fut: std::future::Future<Output=Reason> + Send + 'static
    {
        let running = Running::new();
        let context = Context{ };
        let actor_fut = actor_fn(context);
        smol::spawn(async move {
            running.run(actor_fut).await
        }).detach();    

        Actor {
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
        ActorFn: Fn(Context) -> Fut + 'static,
        Fut: std::future::Future<Output=Reason> + Send + 'static,
    {
        Spec {
            spawn_fn: Box::new(move || {
                Actor::spawn(&actor_fn);
            }),
        }
    }

    /// Spawns an Actor defined by this Spec
    pub fn spawn(&self) {
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
        let spec = Spec::new(|context| async move {
            Reason::Normal
        });

        spec.spawn();
        spec.spawn();

        smol::block_on(async {
            Timer::after(Duration::from_secs(1)).await;
        });
    }
}
