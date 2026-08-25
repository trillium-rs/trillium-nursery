//! Experimental combinators and wrappers for [trillium](https://trillium.rs).
//!
//! Everything in this crate is an api experiment. Residents that prove themselves graduate into
//! the `trillium` crate or a dedicated crate; the rest are removed. Minor versions add, change,
//! and delete freely — pin a version if you depend on one, and expect the stable home of any
//! given item to eventually be elsewhere.
//!
//! Residents:
//!
//! - [`when`] runs a handler only for conns matched by a predicate
//! - [`unhalt`] runs a handler but prevents it from halting the conn
#![forbid(unsafe_code)]
#![deny(
    clippy::dbg_macro,
    missing_copy_implementations,
    rustdoc::missing_crate_level_docs,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    unused_qualifications
)]

mod unhalt;
mod when;

pub use unhalt::{Unhalt, unhalt};
pub use when::{When, when};

// Compile the README as a doctest so its examples stay in sync with the crate.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}
