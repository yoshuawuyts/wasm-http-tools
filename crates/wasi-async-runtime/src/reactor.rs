use super::polling::{EventKey, Poller};

use std::collections::HashMap;
use std::task::Poll;
use std::task::Waker;
use std::{cell::RefCell, rc::Rc};
use wasi::io::poll::Pollable;

/// Manage async system resources for WASI 0.2
#[derive(Debug, Clone)]
pub struct Reactor {
    inner: Rc<RefCell<InnerReactor>>,
}

/// The private, internal `Reactor` implementation - factored out so we can take
/// a lock of the whole.
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

    /// Block until new events are ready. Calls the respective wakers once done.
    ///
    /// # On Wakers and single-threaded runtimes
    ///
    /// At first glance it might seem silly that this goes through the motions
    /// of calling the wakers. The main waker we create here is a `noop` waker:
    /// it does nothing. However, it is common and encouraged to use wakers to
    /// distinguish between events. Concurrency primitives may construct their
    /// own wakers to keep track of identity and wake more precisely. We do not
    /// control the wakers construted by other libraries, and it is for this
    /// reason that we have to call all the wakers - even if by default they
    /// will do nothing.
    pub(crate) fn block_until(&self) {
        let mut reactor = self.inner.borrow_mut();
        for key in reactor.poller.block_until() {
            match reactor.wakers.get(&key) {
                Some(waker) => waker.wake_by_ref(),
                None => {
                    let current_keys: Vec<EventKey> = reactor
                        .poller
                        .targets
                        .iter()
                        .map(|(key, _)| EventKey(key as u32))
                        .collect();
                    panic!("tried to wake the waker for `{key:?}`, but only the keys `{current_keys:?}` are currently present in the poll set")
                }
            }
        }
    }

    /// Wait for the pollable to resolve
    pub async fn wait_for(&self, pollable: Pollable) {
        let mut reactor = self.inner.borrow_mut();
        let key = reactor.poller.insert(pollable);
        drop(reactor); // NOTE: makes sure we don't hold the lock across the .await

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
