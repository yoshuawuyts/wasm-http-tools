use super::polling::{EventKey, Poller};
use std::{cell::RefCell, rc::Rc};
use wasi::io::poll::Pollable;

/// An async executor, converting from WASI poll instances to
#[derive(Debug)]
pub struct Reactor {
    poller: Rc<RefCell<Poller>>,
}

impl Reactor {
    /// Create a new instance of `Reactor`
    pub(crate) fn new() -> Self {
        Self {
            poller: Rc::new(RefCell::new(Poller::new())),
        }
    }

    /// Block until new events are ready.
    pub(crate) fn block_until(&self) {
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
