
/// Mailbox can hold messages of type T
pub struct Mailbox<T> {
    _t: std::marker::PhantomData<T>,
}

pub struct Memory { }

pub const MEM: Memory = Memory { };

impl Memory {
    /// Reserves capacity for `count` of T
    pub fn reserve<T>(self, count: usize) -> Self {
        self
    }

    /// Executes the closure `f` and consumes self
    pub fn with(self, f: impl Fn(Self)) {
        f(self)
    }
}

struct A;
struct B;

fn example() {
    MEM.reserve::<A>(5)
        .reserve::<B>(5);
}