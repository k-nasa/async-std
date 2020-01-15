//! Internal types for coreio.
//!
//! This module is a port of `libcore/io/coreio.rs`,and contains internal types for `print`/`eprint`.

use crate::io::{coreerr, coreout};
use crate::prelude::*;
use core::fmt;

#[doc(hidden)]
pub async fn _print(args: fmt::Arguments<'_>) {
    if let Err(e) = coreout().write_fmt(args).await {
        panic!("failed printing to coreout: {}", e);
    }
}

#[doc(hidden)]
pub async fn _eprint(args: fmt::Arguments<'_>) {
    if let Err(e) = coreerr().write_fmt(args).await {
        panic!("failed printing to coreerr: {}", e);
    }
}
