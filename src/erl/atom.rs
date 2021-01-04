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
//! Example:
//! 
//! ```rust
//! #[macro_use] extern crate thespian;
//! use thespian::atom;
//! 
//! atom!(A);                       // private atom named A
//! atom!(pub B);                   // public atom named B
//! 
//! pub mod other {
//!     use thespian::atom;
//! 
//!     atom!(pub A);               // another atom named A
//! }
//! 
//! fn main() {
//!     assert_eq!(A, A);                       // atoms equal themselves
//!     assert_ne!(A, B);                       // but don't equal each other
//!     assert_ne!(A, other::A);                // not equal: same names but different modules
//!
//!     assert_eq!("rust_out::other::A", other::A.name());        // full name of the atom
//!     assert_eq!("A", other::A.short_name()); // short name of atom
//! }
//! ```

/// A symbolic atom with O(1) equality checks
#[derive(Debug, Eq, Copy, Clone)]
pub struct Atom(
    pub &'static str,
);

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        self.0 as *const str == other.0 as *const str
    }
}

impl Atom {
    /// Returns the full module-scoped name of the Atom
    pub fn name(&self) -> &'static str {
        self.0
    }

    /// Returns the short name (sans module) of the Atom
    pub fn short_name(&self) -> &'static str {
        self.0.rsplit(":").next().unwrap()
    }
}

#[macro_export]
macro_rules! atom {
    ( $visibility:vis $id:ident ) => {
        $visibility const $id: $crate::erl::atom::Atom =
            $crate::erl::atom::Atom(std::concat!(std::module_path!(), "::", std::stringify!($id)));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    atom!(A);
    atom!(B);

    pub mod other {
        atom!(pub A);
    }

    /// An atom equals itself but no others.
    /// Atoms defined in other modules with the same name are not equal.
    #[test]
    fn equality() {
        assert_eq!(A, A);

        assert_ne!(A, B);
        assert_ne!(A, other::A);
    }


    #[test]
    fn naming() {
        // short names
        assert_eq!("A", A.short_name());
        assert_eq!("B", B.short_name());
        assert_eq!("A", other::A.short_name());

        // names
        assert_eq!("thespian::erl::atom::test::A", A.name());
    }

    #[test]
    fn copying() {
        let a_clone = A.clone();
        assert_eq!(a_clone, A);
    }
}
