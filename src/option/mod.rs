//! The Rust core optional value type
//!
//! This module provides the `Option<T>` type for returning and
//! propagating optional values.

mod from_stream;

#[doc(inline)]
pub use core::option::Option;

cfg_unstable! {
    mod product;
    mod sum;
}
