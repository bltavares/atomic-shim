//! Atomic types
//!
//! Atomic types provide primitive shared-memory communication between threads, and are the building blocks of other concurrent types.
//!
//! This module defines atomic versions of a select number of primitive types, including AtomicBool, AtomicIsize, AtomicUsize, AtomicI8, AtomicU16, etc. Atomic types present operations that, when used correctly, synchronize updates between threads.
//!
//! Each method takes an Ordering which represents the strength of the memory barrier for that operation. These orderings are the same as the C++20 atomic orderings. For more information see the nomicon.
//!
//! Atomic variables are safe to share between threads (they implement Sync) but they do not themselves provide the mechanism for sharing and follow the threading model of Rust. The most common way to share an atomic variable is to put it into an Arc (an atomically-reference-counted shared pointer).
//!
//! Atomic types may be stored in static variables, initialized using the constant initializers like AtomicBool::new. Atomic statics are often used for lazy global initialization.
//!
//! # Portability
//!
//! All atomic types in this module are guaranteed to be lock-free if they're available. This means they don't internally acquire a global mutex. Atomic types and operations are not guaranteed to be wait-free. This means that operations like fetch_or may be implemented with a compare-and-swap loop.
//!
//! Atomic operations may be implemented at the instruction layer with larger-size atomics. For example some platforms use 4-byte atomic instructions to implement AtomicI8. Note that this emulation should not have an impact on correctness of code, it's just something to be aware of.
//!
//! The atomic types in this module may not be available on all platforms. The atomic types here are all widely available, however, and can generally be relied upon existing. Some notable exceptions are:
//!
//!  - PowerPC and MIPS platforms with 32-bit pointers do not have AtomicU64 or AtomicI64 types.
//!  - ARM platforms like armv5te that aren't for Linux do not have any atomics at all.
//!  - ARM targets with thumbv6m do not have atomic operations at all.
//!
//! Note that future platforms may be added that also do not have support for some atomic operations. Maximally portable code will want to be careful about which atomic types are used. AtomicUsize and AtomicIsize are generally the most portable, but even then they're not available everywhere. For reference, the std library requires pointer-sized atomics, although core does not.
//!
//! Currently you'll need to use #[cfg(target_arch)] primarily to conditionally compile in code with atomics. There is an unstable #[cfg(target_has_atomic)] as well which may be stabilized in the future.
//! Examples
//!
//! A simple spinlock:
//!
//! ```
//! use std::sync::Arc;
//! use std::sync::atomic::Ordering;
//! use std::thread;
//! use atomic_shim::AtomicU64;
//!
//! fn main() {
//!     let spinlock = Arc::new(AtomicU64::new(1));
//!
//!     let spinlock_clone = spinlock.clone();
//!     let thread = thread::spawn(move|| {
//!         spinlock_clone.store(0, Ordering::SeqCst);
//!     });
//!
//!     // Wait for the other thread to release the lock
//!     while spinlock.load(Ordering::SeqCst) != 0 {}
//!
//!     if let Err(panic) = thread.join() {
//!         println!("Thread had an error: {:?}", panic);
//!     }
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
