//! WASI-based HTTP client library
//!
//! # Examples
//!
//! ```
//! // tbi
//! ```

#![forbid(rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, future_incompatible, unreachable_pub)]

mod polling;

pub use polling::*;

/// An async executor, converting from WASI poll instances to
#[derive(Debug)]
pub struct Reactor {
    poller: Poller,
}
