//! Counts the allocations a thread makes, so a test can prove that code
//! allocates nothing.
//!
//! Install [`CountingAllocator`] as a test binary's global allocator, then
//! measure a closure with [`count`]:
//!
//! ```
//! use nishiki_alloc_count::{CountingAllocator, count};
//!
//! #[global_allocator]
//! static ALLOCATOR: CountingAllocator = CountingAllocator;
//!
//! let buffer: Vec<u8> = Vec::with_capacity(16);
//! assert_eq!(count(|| buffer.len()), 0);
//! assert_eq!(count(|| Vec::<u8>::with_capacity(16).capacity()), 1);
//! ```
//!
//! # Why this crate holds unsafe code
//!
//! nishiki forbids unsafe code. Counting allocations needs a global allocator,
//! and [`GlobalAlloc`] is an unsafe trait: no safe implementation exists. The
//! reason is not performance; the allocator forwards every call to
//! [`System`] unchanged and only counts. An external crate could do the
//! same, but nishiki depends on `memchr` alone and the operator chose no new
//! dependency (2026-11-03). Google's Rust style guide asks for unsafe code to
//! sit behind a safe abstraction in a separate crate with few dependencies;
//! this is that crate. It is never published, it depends on nothing but
//! `std`, and its tests run under Miri.
//!
//! Counts are per thread, so tests that run in parallel do not disturb one
//! another. An allocation made while a thread is being torn down, after its
//! counter is gone, is not counted.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

thread_local! {
    /// Allocations this thread has made since it started.
    static ALLOCATIONS: Cell<usize> = const { Cell::new(0) };
}

/// The system allocator, counting each allocation, zeroed allocation and
/// reallocation on the thread that asks for it.
///
/// Counts are only kept while it is the global allocator.
#[derive(Clone, Copy, Debug, Default)]
pub struct CountingAllocator;

/// Counts one allocation on this thread.
fn record() {
    let _ = ALLOCATIONS.try_with(|count| count.set(count.get().saturating_add(1)));
}

/// This thread's allocations so far.
fn allocations() -> usize {
    ALLOCATIONS.try_with(Cell::get).unwrap_or(0)
}

/// How many allocations this thread made while `f` ran.
///
/// Allocations on other threads, including threads `f` starts, are not
/// counted. The count is 0 unless [`CountingAllocator`] is the global
/// allocator.
pub fn count<R>(f: impl FnOnce() -> R) -> usize {
    let before = allocations();
    let result = f();
    let made = allocations().saturating_sub(before);
    drop(result);
    made
}

#[expect(
    unsafe_code,
    reason = "GlobalAlloc is an unsafe trait, so counting allocations has no \
              safe implementation; see the crate docs"
)]
// SAFETY: each method forwards its arguments unchanged to `System`, which
// meets the `GlobalAlloc` contract, and the counting touches only a
// thread-local integer, which no allocation depends on.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record();
        // SAFETY: the caller upholds `alloc`'s contract for `layout`, which
        // passes to System unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        record();
        // SAFETY: the caller upholds `alloc_zeroed`'s contract for `layout`,
        // which passes to System unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        record();
        // SAFETY: `ptr` was allocated by this allocator, so by System, with
        // `layout`, and the caller upholds `realloc`'s contract for
        // `new_size`.
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: `ptr` was allocated by this allocator, so by System, with
        // `layout`.
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::{CountingAllocator, count};

    #[global_allocator]
    static ALLOCATOR: CountingAllocator = CountingAllocator;

    #[test]
    fn nothing_allocated_counts_zero() {
        let buffer: Vec<u8> = Vec::with_capacity(8);
        assert_eq!(count(|| black_box(buffer.capacity())), 0);
    }

    #[test]
    fn each_allocation_counts_once() {
        assert_eq!(count(|| black_box(Vec::<u8>::with_capacity(8))), 1);
        assert_eq!(count(|| black_box(vec![0_u8; 8])), 1);
        assert_eq!(
            count(|| {
                let mut buffer: Vec<u8> = Vec::with_capacity(1);
                buffer.reserve(64);
                black_box(buffer)
            }),
            2
        );
    }

    #[test]
    fn a_thread_counts_its_own_allocations() {
        let made = std::thread::spawn(|| count(|| black_box(vec![1_u8; 4])))
            .join()
            .unwrap();
        assert_eq!(made, 1);
    }
}
