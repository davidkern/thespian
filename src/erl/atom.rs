//! Symbolic constant atoms with O(1) comparison and copies.

// Re-export the atom!(...) macro.
// Tee Atom struct and tests are defined in this module, but the
// atom! proc-macro must be defined in another crate.
pub use thespian_macros::atom;

/// An Atom is a cheaply clonable instance which represents
/// a named concept. Two atoms are identical if they have
/// identical names.
#[derive(Debug, Eq, Copy, Clone, PartialEq)]
pub struct Atom { pub id: usize }

#[cfg(test)]
mod test {
    use crate as thespian;
    use super::*;

    const A: Atom = atom!(a);
    const B: Atom = atom!(b);
    const LONG: Atom = atom!(abcdefghijklmno);  // longest identifier on 64-bit

    pub mod other {
        use crate as thespian;
        use super::{atom, Atom};

        pub const A: Atom = atom!(a);
    }

    /// An atom equals itself but no others.
    #[test]
    fn equality() {
        // Reflexive
        assert_eq!(A, A);

        // Equality between instances
        assert_eq!(A, other::A);

        // The longest-named atom (15 characters)
        assert_eq!(LONG, atom!(abcdefghijklmno));

        // Disjoint
        assert_ne!(A, B);
    }


    /// An atom may be cloned and still equals itself.
    #[test]
    fn copying() {
        let a_clone = A.clone();
        assert_eq!(a_clone, A);
    }
}
