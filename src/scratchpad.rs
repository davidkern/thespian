//! # A place for code doodles, explorations and half-baked plans.

//! ## Questions
//!
//! * Is there a way to define a trait with async-like methods, without requiring async traits, given
//!   additional restrictions on the trait usage?
//! * Can this be done in a way which does not required boxing of Futures and what are the limitations?
//!
//! This does not compile, and would require use of the async-std crate (which is
//! probably what we will use after playing around a bit).
//!
//! ```compile_fail
//! trait ATrait {
//!     async fn f(&mut State) -> Reply;
//! }
//!
//! struct State;
//! struct Reply;
//! ```
//!
//! Using traits entirely fails, because implementation of an async block or
//! function creates an opaque type.  Trait functions can not have an opaque type as
//! a return type, so there isn't a way to directly return an `impl Future<...>`.
//!
//! A future can be passed in to a non-async fn on the trait as an argument,
//! however, this doesn't open the possibility of returning the async functionality
//! via the argument, since the computation a future will do has already been set.
//!
//! So dropping traits entirely as unworkable (go grab async-trait if the traits
//! won't be used where compiler inlining would be missed.
//!
//! How about generics? Wrapping a more specific type in the more general type
//! allows for interesting patterns.  But ultimately traits are needed to constrain
//! the generic types so that the outer, generic type, can call the inner, specific
//! type.
//!
//! That leaves composition of structs.
//!
//! This seems promising:
//! ```
//! use std::future::Future;
//!
//! struct Processor;
//!
//! impl Processor
//! {
//!     async fn start(f: impl Future<Output=()>) {
//!     }
//! }
//!
//! struct AlphaProcessor(Processor);
//!
//! impl AlphaProcessor {
//!     async fn start() {
//!         Processor::start(async {
//!         });
//!     }
//! }
//!
//! async fn experiment() {
//!     AlphaProcessor::start().await;
//! }
//! ```
//!
//!
//!
//!

use std::future::Future;

struct Processor;

impl Processor
{
    async fn start(f: impl Future<Output=()>) {
    }
}

struct AlphaProcessor(Processor);

impl AlphaProcessor {
    async fn start() {
        Processor::start(async {
        });
    }
}

async fn experiment() {
    AlphaProcessor::start().await;
}
