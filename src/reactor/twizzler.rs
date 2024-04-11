// SPDX-License-Identifier: MIT OR Apache-2.0

use polling::{AsRawSource, AsSource, BorrowedTwizzlerWaitable};
use polling::{Event, Poller};

use std::fmt;
use std::io::Result;

/// The raw registration into the reactor.
#[doc(hidden)]
pub struct Registration {
    /// Twizzler waitable.
    ///
    /// # Invariant
    ///
    /// This describes a valid Twizzler waitable object, with lifetime static, an invariant we'll uphold vir the unsafe new function.
    raw: BorrowedTwizzlerWaitable<'static>,
}

// Safety: Contains a raw pointer that we never read through, with static lifetime (by assumption).
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
    /// Add this waitable into the reactor.
    ///
    /// # Safety
    ///
    /// The provided object must be valid while this object is alive, 'a must outlive Self.
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
