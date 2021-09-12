use crossbeam_utils::sync::ShardedLock;
use std::sync::atomic::Ordering;

/// An integer type which can be safely shared between threads.
#[derive(Debug, Default)]
pub struct AtomicU64 {
    value: ShardedLock<u64>,
}

impl AtomicU64 {
    /// Creates a new atomic integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use atomic_shim::AtomicU64;
    /// let atomic_forty_two = AtomicU64::new(42);
    /// ```
    pub fn new(v: u64) -> Self {
        Self {
            value: ShardedLock::new(v),
        }
    }

    /// Returns a mutable reference to the underlying integer.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicU64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicU64;
    ///
    /// let mut some_var = AtomicU64::new(10);
    /// assert_eq!(*some_var.get_mut(), 10);
    /// *some_var.get_mut() = 5;
    /// assert_eq!(some_var.load(Ordering::SeqCst), 5);
    /// ```
    pub fn get_mut(&mut self) -> &mut u64 {
        self.value.get_mut().unwrap()
    }

    /// Consumes the atomic and returns the contained value.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicU64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::AtomicU64;
    /// let some_var = AtomicU64::new(5);
    /// assert_eq!(some_var.into_inner(), 5);
    /// ```
    pub fn into_inner(self) -> u64 {
        self.value.into_inner().unwrap()
    }

    /// Loads a value from the atomic integer.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicU64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicU64;
    /// let some_var = AtomicU64::new(5);
    /// assert_eq!(some_var.load(Ordering::Relaxed), 5);
    /// ```
    pub fn load(&self, _: Ordering) -> u64 {
        *self.value.read().unwrap()
    }

    /// Stores a value into the atomic integer.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicU64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicU64;
    ///
    /// let some_var = AtomicU64::new(5);
    /// some_var.store(10, Ordering::Relaxed);
    /// assert_eq!(some_var.load(Ordering::Relaxed), 10);
    /// ```
    pub fn store(&self, value: u64, _: Ordering) {
        let mut lock = self.value.write().unwrap();
        *lock = value;
    }

    /// Stores a value into the atomic integer, returning the previous value.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicU64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicU64;
    ///
    /// let some_var = AtomicU64::new(5);
    /// assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);
    /// ```
    pub fn swap(&self, value: u64, _: Ordering) -> u64 {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        *lock = value;
        prev
    }

    /// Stores a value into the atomic integer if the current value is the same as the current value.
    ///
    /// The return value is always the previous value. If it is equal to current, then the value was updated.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicU64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicU64;
    ///
    /// let some_var = AtomicU64::new(5);
    /// assert_eq!(some_var.compare_and_swap(5, 10, Ordering::Relaxed), 5);
    /// assert_eq!(some_var.load(Ordering::Relaxed), 10);
    /// assert_eq!(some_var.compare_and_swap(6, 12, Ordering::Relaxed), 10);
    /// assert_eq!(some_var.load(Ordering::Relaxed), 10);
    /// ```
    pub fn compare_and_swap(&self, current: u64, new: u64, _: Ordering) -> u64 {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        if prev == current {
            *lock = new;
        };
        prev
    }

    /// Stores a value into the atomic integer if the current value is the same as the current value.
    ///
    /// The return value is a result indicating whether the new value was written and containing the previous value. On success this value is guaranteed to be equal to current.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicU64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicU64;
    ///
    /// let some_var = AtomicU64::new(5);
    /// assert_eq!(some_var.compare_exchange(5, 10,
    ///                                      Ordering::Acquire,
    ///                                      Ordering::Relaxed),
    ///            Ok(5));
    /// assert_eq!(some_var.load(Ordering::Relaxed), 10);
    /// assert_eq!(some_var.compare_exchange(6, 12,
    ///                                      Ordering::SeqCst,
    ///                                      Ordering::Acquire),
    ///            Err(10));
    /// assert_eq!(some_var.load(Ordering::Relaxed), 10);
    /// ```
    pub fn compare_exchange(
        &self,
        current: u64,
        new: u64,
        _: Ordering,
        _: Ordering,
    ) -> Result<u64, u64> {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        if prev == current {
            *lock = new;
            Ok(current)
        } else {
            Err(prev)
        }
    }

    /// Stores a value into the atomic integer if the current value is the same as the current value.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicU64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicU64;
    ///
    /// let val = AtomicU64::new(4);
    /// let mut old = val.load(Ordering::Relaxed);
    /// loop {
    ///     let new = old * 2;
    ///     match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
    ///         Ok(_) => break,
    ///         Err(x) => old = x,
    ///     }
    /// }
    /// ```
    pub fn compare_exchange_weak(
        &self,
        current: u64,
        new: u64,
        success: Ordering,
        failure: Ordering,
    ) -> Result<u64, u64> {
        self.compare_exchange(current, new, success, failure)
    }

    /// Adds to the current value, returning the previous value.
    ///
    /// This operation wraps around on overflow.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicU64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicU64;
    ///
    /// let foo = AtomicU64::new(0);
    /// assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);
    /// assert_eq!(foo.load(Ordering::SeqCst), 10);
    /// ```
    pub fn fetch_add(&self, val: u64, _: Ordering) -> u64 {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        *lock = prev.wrapping_add(val);
        prev
    }

    /// Subtracts from the current value, returning the previous value.
    ///
    /// This operation wraps around on overflow.
    ///    
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicU64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicU64;
    ///
    /// let foo = AtomicU64::new(20);
    /// assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);
    /// assert_eq!(foo.load(Ordering::SeqCst), 10);
    /// ```
    pub fn fetch_sub(&self, val: u64, _: Ordering) -> u64 {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        *lock = prev.wrapping_sub(val);
        prev
    }

    /// Bitwise "and" with the current value.
    ///
    /// Performs a bitwise "and" operation on the current value and the argument val, and sets the new value to the result.
    /// Returns the previous value.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicU64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicU64;
    ///
    /// let foo = AtomicU64::new(0b101101);
    /// assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);
    /// assert_eq!(foo.load(Ordering::SeqCst), 0b100001);
    /// ```
    pub fn fetch_and(&self, val: u64, _: Ordering) -> u64 {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        *lock = prev & val;
        prev
    }

    /// Bitwise "nand" with the current value.
    ///
    /// Performs a bitwise "nand" operation on the current value and the argument val, and sets the new value to the result.
    /// Returns the previous value.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicU64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicU64;
    ///
    /// let foo = AtomicU64::new(0x13);
    /// assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);
    /// assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));
    /// ```
    pub fn fetch_nand(&self, val: u64, _: Ordering) -> u64 {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        *lock = !(prev & val);
        prev
    }

    /// Bitwise "or" with the current value.
    ///
    /// Performs a bitwise "or" operation on the current value and the argument val, and sets the new value to the result.
    /// Returns the previous value.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicU64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicU64;
    ///
    /// let foo = AtomicU64::new(0b101101);
    /// assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);
    /// assert_eq!(foo.load(Ordering::SeqCst), 0b111111);
    /// ```
    pub fn fetch_or(&self, val: u64, _: Ordering) -> u64 {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        *lock = prev | val;
        prev
    }

    /// Bitwise "xor" with the current value.
    ///
    /// Performs a bitwise "xor" operation on the current value and the argument val, and sets the new value to the result.
    /// Returns the previous value.
    ///
    ///     
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicU64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    ///  # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicU64;
    /// let foo = AtomicU64::new(0b101101);
    /// assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);
    /// assert_eq!(foo.load(Ordering::SeqCst), 0b011110);
    /// ```
    pub fn fetch_xor(&self, val: u64, _: Ordering) -> u64 {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        *lock = prev ^ val;
        prev
    }
}

impl From<u64> for AtomicU64 {
    fn from(value: u64) -> Self {
        AtomicU64::new(value)
    }
}

/// An integer type which can be safely shared between threads.
#[derive(Debug, Default)]
pub struct AtomicI64 {
    value: ShardedLock<i64>,
}

impl AtomicI64 {
    /// Creates a new atomic integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use atomic_shim::AtomicI64;
    /// let atomic_forty_two = AtomicI64::new(42);
    /// ```
    pub fn new(v: i64) -> Self {
        Self {
            value: ShardedLock::new(v),
        }
    }

    /// Returns a mutable reference to the underlying integer.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicI64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicI64;
    ///
    /// let mut some_var = AtomicI64::new(10);
    /// assert_eq!(*some_var.get_mut(), 10);
    /// *some_var.get_mut() = 5;
    /// assert_eq!(some_var.load(Ordering::SeqCst), 5);
    /// ```
    pub fn get_mut(&mut self) -> &mut i64 {
        self.value.get_mut().unwrap()
    }

    /// Consumes the atomic and returns the contained value.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicI64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::AtomicI64;
    /// let some_var = AtomicI64::new(5);
    /// assert_eq!(some_var.into_inner(), 5);
    /// ```
    pub fn into_inner(self) -> i64 {
        self.value.into_inner().unwrap()
    }

    /// Loads a value from the atomic integer.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicI64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicI64;
    /// let some_var = AtomicI64::new(5);
    /// assert_eq!(some_var.load(Ordering::Relaxed), 5);
    /// ```
    pub fn load(&self, _: Ordering) -> i64 {
        *self.value.read().unwrap()
    }

    /// Stores a value into the atomic integer.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicI64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicI64;
    ///
    /// let some_var = AtomicI64::new(5);
    /// some_var.store(10, Ordering::Relaxed);
    /// assert_eq!(some_var.load(Ordering::Relaxed), 10);
    /// ```
    pub fn store(&self, value: i64, _: Ordering) {
        let mut lock = self.value.write().unwrap();
        *lock = value;
    }

    /// Stores a value into the atomic integer, returning the previous value.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicI64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicI64;
    ///
    /// let some_var = AtomicI64::new(5);
    /// assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);
    /// ```
    pub fn swap(&self, value: i64, _: Ordering) -> i64 {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        *lock = value;
        prev
    }

    /// Stores a value into the atomic integer if the current value is the same as the current value.
    ///
    /// The return value is always the previous value. If it is equal to current, then the value was updated.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicI64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicI64;
    ///
    /// let some_var = AtomicI64::new(5);
    /// assert_eq!(some_var.compare_and_swap(5, 10, Ordering::Relaxed), 5);
    /// assert_eq!(some_var.load(Ordering::Relaxed), 10);
    /// assert_eq!(some_var.compare_and_swap(6, 12, Ordering::Relaxed), 10);
    /// assert_eq!(some_var.load(Ordering::Relaxed), 10);
    /// ```
    pub fn compare_and_swap(&self, current: i64, new: i64, _: Ordering) -> i64 {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        if prev == current {
            *lock = new;
        };
        prev
    }

    /// Stores a value into the atomic integer if the current value is the same as the current value.
    ///
    /// The return value is a result indicating whether the new value was written and containing the previous value. On success this value is guaranteed to be equal to current.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicI64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicI64;
    ///
    /// let some_var = AtomicI64::new(5);
    /// assert_eq!(some_var.compare_exchange(5, 10,
    ///                                      Ordering::Acquire,
    ///                                      Ordering::Relaxed),
    ///            Ok(5));
    /// assert_eq!(some_var.load(Ordering::Relaxed), 10);
    /// assert_eq!(some_var.compare_exchange(6, 12,
    ///                                      Ordering::SeqCst,
    ///                                      Ordering::Acquire),
    ///            Err(10));
    /// assert_eq!(some_var.load(Ordering::Relaxed), 10);
    /// ```
    pub fn compare_exchange(
        &self,
        current: i64,
        new: i64,
        _: Ordering,
        _: Ordering,
    ) -> Result<i64, i64> {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        if prev == current {
            *lock = new;
            Ok(current)
        } else {
            Err(prev)
        }
    }

    /// Stores a value into the atomic integer if the current value is the same as the current value.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicI64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicI64;
    ///
    /// let val = AtomicI64::new(4);
    /// let mut old = val.load(Ordering::Relaxed);
    /// loop {
    ///     let new = old * 2;
    ///     match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
    ///         Ok(_) => break,
    ///         Err(x) => old = x,
    ///     }
    /// }
    /// ```
    pub fn compare_exchange_weak(
        &self,
        current: i64,
        new: i64,
        success: Ordering,
        failure: Ordering,
    ) -> Result<i64, i64> {
        self.compare_exchange(current, new, success, failure)
    }

    /// Adds to the current value, returning the previous value.
    ///
    /// This operation wraps around on overflow.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicI64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicI64;
    ///
    /// let foo = AtomicI64::new(0);
    /// assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);
    /// assert_eq!(foo.load(Ordering::SeqCst), 10);
    /// ```
    pub fn fetch_add(&self, val: i64, _: Ordering) -> i64 {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        *lock = prev.wrapping_add(val);
        prev
    }

    /// Subtracts from the current value, returning the previous value.
    ///
    /// This operation wraps around on overflow.
    ///    
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicI64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicI64;
    ///
    /// let foo = AtomicI64::new(20);
    /// assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);
    /// assert_eq!(foo.load(Ordering::SeqCst), 10);
    /// ```
    pub fn fetch_sub(&self, val: i64, _: Ordering) -> i64 {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        *lock = prev.wrapping_sub(val);
        prev
    }

    /// Bitwise "and" with the current value.
    ///
    /// Performs a bitwise "and" operation on the current value and the argument val, and sets the new value to the result.
    /// Returns the previous value.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicI64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicI64;
    ///
    /// let foo = AtomicI64::new(0b101101);
    /// assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);
    /// assert_eq!(foo.load(Ordering::SeqCst), 0b100001);
    /// ```
    pub fn fetch_and(&self, val: i64, _: Ordering) -> i64 {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        *lock = prev & val;
        prev
    }

    /// Bitwise "nand" with the current value.
    ///
    /// Performs a bitwise "nand" operation on the current value and the argument val, and sets the new value to the result.
    /// Returns the previous value.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicI64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicI64;
    ///
    /// let foo = AtomicI64::new(0x13);
    /// assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);
    /// assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));
    /// ```
    pub fn fetch_nand(&self, val: i64, _: Ordering) -> i64 {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        *lock = !(prev & val);
        prev
    }

    /// Bitwise "or" with the current value.
    ///
    /// Performs a bitwise "or" operation on the current value and the argument val, and sets the new value to the result.
    /// Returns the previous value.
    ///
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicI64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicI64;
    ///
    /// let foo = AtomicI64::new(0b101101);
    /// assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);
    /// assert_eq!(foo.load(Ordering::SeqCst), 0b111111);
    /// ```
    pub fn fetch_or(&self, val: i64, _: Ordering) -> i64 {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        *lock = prev | val;
        prev
    }

    /// Bitwise "xor" with the current value.
    ///
    /// Performs a bitwise "xor" operation on the current value and the argument val, and sets the new value to the result.
    /// Returns the previous value.
    ///
    ///     
    /// It ignores the Ordering argument, but it is required for compatibility with `std::sync::AtomicI64`
    ///
    /// # Panics
    ///
    /// Panics if the Mutex is poisoned
    ///
    ///  # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    /// use atomic_shim::AtomicI64;
    /// let foo = AtomicI64::new(0b101101);
    /// assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);
    /// assert_eq!(foo.load(Ordering::SeqCst), 0b011110);
    /// ```
    pub fn fetch_xor(&self, val: i64, _: Ordering) -> i64 {
        let mut lock = self.value.write().unwrap();
        let prev = *lock;
        *lock = prev ^ val;
        prev
    }
}

impl From<i64> for AtomicI64 {
    fn from(value: i64) -> Self {
        AtomicI64::new(value)
    }
}
