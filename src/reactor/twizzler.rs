// SPDX-License-Identifier: MIT OR Apache-2.0

use polling::{AsRawSource, AsSource, BorrowedTwizzlerWaitable};
use polling::{Event, Poller};

use std::fmt;
use std::io::Result;

/// The raw registration into the reactor.
#[doc(hidden)]
pub struct Registration {
    /// Raw file descriptor on Unix.
    ///
    /// # Invariant
    ///
    /// This describes a valid file descriptor that has not been `close`d. It will not be
    /// closed while this object is alive.
    raw: BorrowedTwizzlerWaitable<'static>,
}

unsafe impl Sync for Registration {}

impl fmt::Debug for Registration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.raw, f)
    }
}

impl AsRawSource for &Registration {
    fn raw(&self) -> &BorrowedTwizzlerWaitable<'static> {
        &self.raw
    }
}

impl AsSource for &Registration {
    fn source(&self) -> &BorrowedTwizzlerWaitable<'static> {
        &self.raw
    }
}

impl Registration {
    /// Add this file descriptor into the reactor.
    ///
    /// # Safety
    ///
    /// The provided file descriptor must be valid and not be closed while this object is alive.
    pub(crate) unsafe fn new<'a>(f: BorrowedTwizzlerWaitable<'a>) -> Self {
        Self {
            raw: core::mem::transmute(f),
        }
    }

    /// Registers the object into the reactor.
    #[inline]
    pub(crate) fn add(&self, poller: &Poller, token: usize) -> Result<()> {
        // SAFETY: This object's existence validates the invariants of Poller::add
        unsafe { poller.add(self, Event::none(token)) }
    }

    /// Re-registers the object into the reactor.
    #[inline]
    pub(crate) fn modify(&self, poller: &Poller, interest: Event) -> Result<()> {
        poller.modify(self, interest)
    }

    /// Deregisters the object from the reactor.
    #[inline]
    pub(crate) fn delete(&self, poller: &Poller) -> Result<()> {
        poller.delete(self)
    }
}
