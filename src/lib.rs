//! One-shot locks that panic instead of (dead)locking on contention.
//!
//! These locks allow no contention and panic instead of blocking on `lock` if they are already locked.
//! This is useful in situations where contention would be a bug,
//! such as in single-threaded programs that would deadlock on contention.
//!
//! See the [`RawOneShotMutex`] and [`RawOneShotRwLock`] types for more information.

#![no_std]

mod mutex;
mod rwlock;
mod unsync;

pub use mutex::{OneShotMutex, OneShotMutexGuard, RawOneShotMutex};
pub use rwlock::{
    OneShotRwLock, OneShotRwLockReadGuard, OneShotRwLockUpgradableReadGuard,
    OneShotRwLockWriteGuard, RawOneShotRwLock,
};
pub use unsync::mutex::{RawUnsyncOneShotMutex, UnsyncOneShotMutex, UnsyncOneShotMutexGuard};
pub use unsync::rwlock::{
    RawUnsyncOneShotRwLock, UnsyncOneShotRwLock, UnsyncOneShotRwLockReadGuard,
    UnsyncOneShotRwLockUpgradableReadGuard, UnsyncOneShotRwLockWriteGuard,
};
