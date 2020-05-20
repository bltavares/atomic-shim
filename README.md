# atomic-shim

Atomic types shims for unsupported architectures.

<a href="https://github.com/bltavares/atomic-shim/actions?query=workflow%3AQuickstart+branch%3Amaster">
    <img src="https://img.shields.io/github/workflow/status/bltavares/atomic-shim/Quickstart/master?label=main%20ci" />
</a>
<a href="https://github.com/bltavares/atomic-shim/actions?query=workflow%3ACross-compile+branch%3Amaster">
    <img src="https://img.shields.io/github/workflow/status/bltavares/atomic-shim/Cross-compile/master?label=cross%20ci" />
</a>
<a href="https://crates.io/atomic-shim">
    <img src="https://img.shields.io/crates/v/atomic-shim.svg" />
</a>
<a href="https://doc.rs/atomic-shim">
    <img src="https://docs.rs/atomic-shim/badge.svg" />
</a>

This crate provides shims for `std::sync::AtomicU64` and `std::sync::AtomicI64` for `mips` and `powerpc`.

The `std` primitives are not available on all platforms, and that makes it tricky to write code for `mips`, such as OpenWRT Routers.
This crate provides it's own `AtomicU64` and `AtomicI64`, which can directly replace the `std::sync` structs.

The crate does target detection and on supported architectures it will use `std::sync` structures.
When it detects it is running on unsupported platforms, it fallbacks to the shim implementation, using `crossbeam` Mutex.

For testing purposes, and for other reasons, you can replace the default implementation with the Mutex implementation by using the `features = ["mutex"]`

## Usage

Replace any imports of `use std::sync::AtomicU64;` with `use atomic_shim::Atomic64;`

## Installation

Add the dependency to your `Cargo.toml`, and optionally, exposes the `mutex` feature to test without cross-compiling:

```toml
[dependencies]
atomic-shim = "*"

# Optional
#[features]
#mutex = ["atomic-shim/mutex"]
```

## Test

To run tests, it is important to enable the `--features mutex`.

```sh
cargo test --features mutex
```

## Examples

A simple spinlock:

```rust
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::thread;
use atomic_shim::AtomicU64;

fn main() {
    let spinlock = Arc::new(AtomicU64::new(1));

    let spinlock_clone = spinlock.clone();
    let thread = thread::spawn(move|| {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // Wait for the other thread to release the lock
    while spinlock.load(Ordering::SeqCst) != 0 {}

    if let Err(panic) = thread.join() {
        println!("Thread had an error: {:?}", panic);
    }
}
```

Keep a global count of live threads:

```rust
use std::sync::atomic::Ordering;
use atomic_shim::AtomicU64;

let global_thread_count = AtomicU64::new(0);

let old_thread_count = global_thread_count.fetch_add(1, Ordering::SeqCst);
println!("live threads: {}", old_thread_count + 1);
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
