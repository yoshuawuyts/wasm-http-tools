//! A single-threaded native runtime for WASI 0.2

mod polling;
mod reactor;

pub use polling::{EventKey, Poller};
pub use reactor::Reactor;

use core::pin::pin;
use std::future::Future;
use std::ptr;
use std::task::{Context, Poll, RawWaker, RawWakerVTable};
use std::{marker::PhantomData, task::Waker};

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
        F: FnOnce(Reactor) -> Fut,
        Fut: Future,
    {
        // Construct the reactor
        let reactor = Reactor::new();

        // Create the future and pin it so it can be polled
        let fut = (f)(reactor.clone());
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
