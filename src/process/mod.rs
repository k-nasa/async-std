//! A module for working with processes.
//!
//! This module is mostly concerned with spawning and interacting with child processes, but it also
//! provides abort and exit for terminating the current process.
//!
//! This is an async version of [`core::process`].
//!
//! [`core::process`]: https://doc.rust-lang.org/core/process/index.html

// Re-export structs.
pub use core::process::{ExitStatus, Output};

// Re-export functions.
pub use core::process::{abort, exit, id};
