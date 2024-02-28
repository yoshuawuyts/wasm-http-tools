//! WASI-based HTTP client library
//!
//! # Examples
//!
//! ```text
//! // tbi
//! ```

#![forbid(rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, future_incompatible, unreachable_pub)]

mod polling;

use std::ptr;
use std::{cell::RefCell, marker::PhantomData, rc::Rc, task::Waker};

pub use polling::*;
use wasi::io::poll::Pollable;

/// An async executor, converting from WASI poll instances to
#[derive(Debug)]
pub struct Reactor {
    poller: Rc<RefCell<Poller>>,
}

impl Reactor {
    /// Create a new instance of `Reactor`
    fn new() -> Self {
        Self {
            poller: Rc::new(RefCell::new(Poller::new())),
        }
    }

    /// Block until new events are ready.
    fn block_until(&self) {
        self.poller.borrow_mut().block_until();
    }

    /// Register interest in a `Pollable``.
    pub fn register(&self, pollable: Pollable) -> EventKey {
        self.poller.borrow_mut().insert(pollable)
    }

    /// Deregister interest in a `Pollable`
    pub fn deregister(&self, key: EventKey) {
        self.poller.borrow_mut().remove(key);
    }
}

use core::pin::pin;
use std::future::Future;
use std::task::{Context, Poll, RawWaker, RawWakerVTable};

/// An asynchronous runtime for WASI 0.2.0
#[derive(Debug)]
pub struct Runtime {
    _data: PhantomData<()>,
}

impl Runtime {
    /// Construct a new instance of `Runtime`
    pub fn new() -> Self {
        Self { _data: PhantomData }
    }

    /// Start the event loop
    pub fn run<F, Fut>(self, f: F) -> Fut::Output
    where
        F: FnOnce(&Reactor) -> Fut,
        Fut: Future,
    {
        // Construct the reactor
        let reactor = Reactor::new();

        // Create the future and pin it so it can be polled
        let fut = (f)(&reactor);
        let mut fut = pin!(fut);

        // Create a new context to be passed to the future.
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        // Either the future completes and we return, or some IO is happening
        // and we wait.
        loop {
            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(res) => return res,
                Poll::Pending => reactor.block_until(),
            }
        }
    }
}

/// Construct a new no-op waker
fn noop_waker() -> Waker {
    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        // Cloning just returns a new no-op raw waker
        |_| RAW,
        // `wake` does nothing
        |_| {},
        // `wake_by_ref` does nothing
        |_| {},
        // Dropping does nothing as we don't allocate anything
        |_| {},
    );
    const RAW: RawWaker = RawWaker::new(ptr::null(), &VTABLE);

    // SAFETY: all fields are no-ops, so this is safe
    unsafe { Waker::from_raw(RAW) }
}
