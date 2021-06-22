pub mod atom;
pub mod mailbox;
pub mod process;

trait Apply<T, U> {
    fn apply(self, t: T) -> U; 
}

