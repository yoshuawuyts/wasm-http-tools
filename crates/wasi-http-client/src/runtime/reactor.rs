use super::polling::{EventKey, Poller};
use std::task::Poll;
use std::{cell::RefCell, rc::Rc};
use wasi::io::poll::Pollable;

/// An async executor, converting from WASI poll instances to
#[derive(Debug, Clone)]
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

    /// Wait for the pollable to resolve
    pub async fn wait_for(&self, pollable: Pollable) {
        let key = self.poller.borrow_mut().insert(pollable);
        std::future::poll_fn(|_cx| -> Poll<()> {
            let poller = self.poller.borrow();
            let pollable = poller.get(&key).unwrap();
            match pollable.ready() {
                true => Poll::Ready(()),
                false => Poll::Pending,
            }
        })
        .await;
        self.poller.borrow_mut().remove(key);
    }
}
