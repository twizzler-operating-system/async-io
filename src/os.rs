//! Platform-specific functionality.

#[cfg(all(unix, not(target_os = "twizzler")))]
pub mod unix;

#[cfg(any(
    target_vendor = "apple",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "dragonfly",
))]
pub mod kqueue;

#[cfg(windows)]
pub mod windows;
