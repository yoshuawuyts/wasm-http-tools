//! This module handles the conversion from `Pollable` -> `Future`. We do this
//! by creating an equivalent implementation to the `polling` crate. Once
//! <https://github.com/smol-rs/polling/issues/102> has been resolved, this module
//! will likely no longer be needed.

use slab::Slab;
use std::mem;
use wasi::io::poll::{poll, Pollable};

/// Waits for I/O events.
#[derive(Debug)]
pub(crate) struct Poller {
    targets: Slab<Pollable>,
    ready_list: Vec<EventKey>,
}

impl Poller {
    /// Create a new instance of `Poller`
    pub(crate) fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Create a new instance of `Poller` with a given capacity
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            targets: Slab::with_capacity(capacity),
            ready_list: Vec::with_capacity(capacity),
        }
    }

    /// Insert a new `Pollable` target into `Poller`
    pub(crate) fn insert(&mut self, target: Pollable) -> EventKey {
        let key = self.targets.insert(target);
        EventKey(key as u32)
    }

    /// Get a `Pollable` if it exists.
    pub(crate) fn get(&self, key: &EventKey) -> Option<&Pollable> {
        self.targets.get(key.0 as usize)
    }

    /// Remove an instance of `Pollable` from `Poller`.
    ///
    /// Returns `None` if no entry was found for `key`.
    pub(crate) fn remove(&mut self, key: EventKey) -> Option<Pollable> {
        self.targets.try_remove(key.0 as usize)
    }

    /// Block the current thread until a new event has triggered.
    ///
    /// This will clear the value of `ready_list`.
    pub(crate) fn block_until(&mut self) {
        let targets: Vec<_> = self.targets.iter().map(|(_, target)| target).collect();
        let ready_list = poll(&targets);

        // SAFETY: Transmute from a `Vec<u32>` to a `Vec<EventKey>`. This is
        // safe because `EventKey` is `#[repr(transparent)]` for `u32`.
        self.ready_list = unsafe { mem::transmute(ready_list) };
    }
}

/// A key representing the
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct EventKey(u32);
