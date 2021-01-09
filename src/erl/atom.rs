//! Symbolic constant atoms with O(1) comparison and copies.
//! 
//! Because of difficulty (impossibility?) of keeping state in Rust macros,
//! Atoms must be defined before they are used.
//! 
//! Definition is via the macro `atom!(IDENT)` which defines an Atom as a
//! constant named `IDENT` in the calling module.  Note that atoms defined
//! with the same `IDENT` in different modules are different atoms! `IDENT`
//! should follow the rules for naming constants: letters [A-Z], underscores,
//! and [0-9] not as the first character.
//! 
//! Internally, atoms are named with their fully-qualified module path.
//! In other words, calling `atom!(pub A)` in the root module of crate "foo"
//! will expand to `pub const A: Atom = Atom { name: "foo:A" };` This ensures
//! that distinctly defined atoms are unique, since it is not possible to have
//! two constants with the same name in the same module.
//! 
//! Comparison of atoms is done by comparing the internal static string pointers,
//! rather than comparing the strings themselves.  Short and full names are
//! provided for display and debugging purposes, however they should not be
//! considered a stable format - in the future the format may be changed.
//!
//! ```

pub use thespian_macros::atom;

#[derive(Debug, Eq, Copy, Clone, PartialEq)]
pub struct Atom { pub id: usize }

/// A symbolic atom with O(1) equality checks
// pub struct Atom(
//     pub &'static str,
// );

// impl PartialEq for Atom {
//     fn eq(&self, other: &Self) -> bool {
//         self.0 as *const str == other.0 as *const str
//     }
// }

// impl Atom {
//     /// Returns the full module-scoped name of the Atom
//     pub fn name(&self) -> &'static str {
//         self.0
//     }

//     /// Returns the short name (sans module) of the Atom
//     pub fn short_name(&self) -> &'static str {
//         self.0.rsplit(":").next().unwrap()
//     }
// }

// #[macro_export]
// macro_rules! atom {
//     ( $visibility:vis $id:ident ) => {
//         $visibility const $id: $crate::erl::atom::Atom =
//             $crate::erl::atom::Atom(std::concat!(std::module_path!(), "::", std::stringify!($id)));
//     }
// }

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
        assert_eq!(A, A);
        assert_eq!(A, other::A);
        assert_eq!(LONG, atom!(abcdefghijklmno));

        assert_ne!(A, B);
    }


    #[test]
    fn copying() {
        let a_clone = A.clone();
        assert_eq!(a_clone, A);
    }
}
