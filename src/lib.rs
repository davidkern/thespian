//! # Redesign
//!
//! GOAL: Simple supervision tree
//! 
//! TODO: Documentation

// TODO: Enable these
// #![deny(missing_docs,
//     missing_debug_implementations, missing_copy_implementations,
//     trivial_casts, trivial_numeric_casts,
//     unsafe_code,
//     unstable_features,
//     unused_import_braces, unused_qualifications)]
use crate as thespian;
use thespian_macros::atom;


pub const A: erl::atom::Atom = atom!(a);

/// actor system
pub mod actor;

/// concepts adapted from Erlang OTP
pub mod erl;

/// refining workers
pub mod next;
