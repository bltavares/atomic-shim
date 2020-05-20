//! Atomic types shims for unsupported architectures
//!
//! This crate provides shims for `std::sync::AtomicU64` and `std::sync::AtomicI64` for `mips` and `powerpc`.
//!
//! The `std` primitives are not available on all platforms, and that makes it tricky to write code for `mips`, such as OpenWRT Routers.
//! This crate provides it's own `AtomicU64` and `AtomicI64`, which can directly replace the `std::sync` structs.
//!
//! The crate does target detection and on supported architectures it will use `std::sync` structures.
//! When it detects it is running on unsupported platforms, it fallbacks to the shim implementation, using `crossbeam` Mutex.
//!
//! For testing purposes, and for other reasons, you can replace the default implementation with the Mutex implementation by using the `features = ["mutex"]`
//!
//! # Usage
//!
//! Replace any imports of `use std::sync::AtomicU64;` with `use atomic_shim::Atomic64;`
//!
//! # Examples
//!
//! A simple spinlock:
//!
//! ```
//! use std::sync::Arc;
//! use std::sync::atomic::Ordering;
//! use std::thread;
//! use atomic_shim::AtomicU64;
//!
//!
//! let spinlock = Arc::new(AtomicU64::new(1));
//!
//! let spinlock_clone = spinlock.clone();
//! let thread = thread::spawn(move|| {
//!     spinlock_clone.store(0, Ordering::SeqCst);
//! });
//!
//! // Wait for the other thread to release the lock
//! while spinlock.load(Ordering::SeqCst) != 0 {}
//!
//! if let Err(panic) = thread.join() {
//!     println!("Thread had an error: {:?}", panic);
//! }
//!```
//!
//! Keep a global count of live threads:
//!
//! ```
//! use std::sync::atomic::Ordering;
//! use atomic_shim::AtomicU64;
//!
//! let global_thread_count = AtomicU64::new(0);
//!
//! let old_thread_count = global_thread_count.fetch_add(1, Ordering::SeqCst);
//! println!("live threads: {}", old_thread_count + 1);
//! ```

#[cfg(not(any(target_arch = "mips", target_arch = "powerpc", feature = "mutex")))]
pub use std::sync::atomic::{AtomicI64, AtomicU64};

#[cfg(any(target_arch = "mips", target_arch = "powerpc", feature = "mutex"))]
mod shim;

#[cfg(any(target_arch = "mips", target_arch = "powerpc", feature = "mutex"))]
pub use shim::{AtomicI64, AtomicU64};
