use super::polling::{EventKey, Poller};

use std::collections::HashMap;
use std::task::Poll;
use std::task::Waker;
use std::{cell::RefCell, rc::Rc};
use wasi::io::poll::Pollable;

/// An async executor, converting from WASI poll instances to
#[derive(Debug, Clone)]
pub struct Reactor {
    inner: Rc<RefCell<InnerReactor>>,
}

#[derive(Debug)]
struct InnerReactor {
    poller: Poller,
    wakers: HashMap<EventKey, Waker>,
}

impl Reactor {
    /// Create a new instance of `Reactor`
    pub(crate) fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerReactor {
                poller: Poller::new(),
                wakers: HashMap::new(),
            })),
        }
    }

    /// Block until new events are ready.
    pub(crate) fn block_until(&self) {
        dbg!();
        let mut reactor = self.inner.borrow_mut();
        for key in reactor.poller.block_until() {
            reactor.wakers[&key].wake_by_ref();
        }
    }

    /// Wait for the pollable to resolve
    pub async fn wait_for(&self, pollable: Pollable) {
        let mut reactor = self.inner.borrow_mut();
        let key = reactor.poller.insert(pollable);
        drop(reactor); // NOTE: make sure we don't hold the lock across the .await

        std::future::poll_fn(|cx| -> Poll<()> {
            let mut reactor = self.inner.borrow_mut();
            let waker = cx.waker();
            reactor.wakers.insert(key, waker.clone());

            let pollable = reactor.poller.get(&key).unwrap();
            match pollable.ready() {
                true => Poll::Ready(()),
                false => Poll::Pending,
            }
        })
        .await;

        let mut reactor = self.inner.borrow_mut();
        reactor.poller.remove(key);
        reactor.wakers.remove(&key);
    }
}
