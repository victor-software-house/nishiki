//! Counts the allocations a thread makes, and their bytes.
//!
//! A test can then prove that code allocates nothing, and a benchmark can
//! report what it allocates.
//!
//! Install [`CountingAllocator`] as a binary's global allocator, then
//! measure a closure with [`count`] or [`measure`]:
//!
//! ```
//! use nishiki_alloc_count::{CountingAllocator, count, measure};
//!
//! #[global_allocator]
//! static ALLOCATOR: CountingAllocator = CountingAllocator;
//!
//! let buffer: Vec<u8> = Vec::with_capacity(16);
//! assert_eq!(count(|| buffer.len()), 0);
//! assert_eq!(count(|| Vec::<u8>::with_capacity(16).capacity()), 1);
//!
//! let usage = measure(|| Vec::<u8>::with_capacity(16));
//! assert_eq!((usage.allocations, usage.bytes, usage.peak), (1, 16, 16));
//! ```
//!
//! # The reason for unsafe code
//!
//! nishiki forbids unsafe code. Counting allocations needs a global allocator,
//! and [`GlobalAlloc`] is an unsafe trait: no safe implementation exists.
//!
//! - The reason is not performance. The allocator forwards every call to
//!   [`System`] unchanged and only counts.
//! - An external crate could do the same. But nishiki depends on `memchr`
//!   alone, and the operator chose no new dependency (2026-11-03).
//! - Google's Rust style guide puts unsafe code behind a safe abstraction, in a
//!   separate crate with few dependencies. This is that crate.
//! - It is never published, it depends on nothing but `std`, and its tests run
//!   under Miri.
//!
//! Counts are per thread, so tests that run in parallel do not disturb one
//! another. An allocation made while a thread is being torn down, after its
//! counters are gone, is not counted. Memory freed on another thread than
//! the one that allocated it lowers the live bytes of the thread that frees
//! it, never below zero.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

thread_local! {
    /// Allocations this thread has made since it started.
    static ALLOCATIONS: Cell<usize> = const { Cell::new(0) };
    /// Bytes this thread has asked for since it started, each reallocation
    /// counted at its new size.
    static BYTES: Cell<usize> = const { Cell::new(0) };
    /// Bytes this thread holds now.
    static LIVE: Cell<usize> = const { Cell::new(0) };
    /// The most bytes this thread has held since the innermost running
    /// [`measure`] started.
    static PEAK: Cell<usize> = const { Cell::new(0) };
}

/// The system allocator, counting each allocation, zeroed allocation and
/// reallocation, and the bytes held, on the thread that asks for it.
///
/// Counts are only kept while it is the global allocator.
#[derive(Clone, Copy, Debug, Default)]
pub struct CountingAllocator;

/// What a closure allocated on its thread, as [`measure`] reports it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Usage {
    /// Allocations, zeroed allocations and reallocations.
    pub allocations: usize,
    /// Bytes requested, each reallocation counted at its new size.
    pub bytes: usize,
    /// The most bytes held at once, above those held when the closure
    /// started.
    pub peak: usize,
}

/// Adds `amount` to a counter of this thread, saturating.
fn add(counter: &'static std::thread::LocalKey<Cell<usize>>, amount: usize) {
    let _ = counter.try_with(|cell| cell.set(cell.get().saturating_add(amount)));
}

/// Reads a counter of this thread; 0 once the thread is being torn down.
fn get(counter: &'static std::thread::LocalKey<Cell<usize>>) -> usize {
    counter.try_with(Cell::get).unwrap_or(0)
}

/// Counts one allocation of `size` bytes on this thread.
fn record_alloc(size: usize) {
    add(&ALLOCATIONS, 1);
    add(&BYTES, size);
    grow(size);
}

/// Adds `size` to this thread's live bytes and raises the peak to match.
fn grow(size: usize) {
    let _ = LIVE.try_with(|live| {
        let now = live.get().saturating_add(size);
        live.set(now);
        let _ = PEAK.try_with(|peak| peak.set(peak.get().max(now)));
    });
}

/// Takes `size` from this thread's live bytes, never below zero.
fn shrink(size: usize) {
    let _ = LIVE.try_with(|live| live.set(live.get().saturating_sub(size)));
}

/// How many allocations this thread made while `f` ran.
///
/// Allocations on other threads, including threads `f` starts, are not
/// counted. The count is 0 unless [`CountingAllocator`] is the global
/// allocator.
pub fn count<R>(f: impl FnOnce() -> R) -> usize {
    measure(f).allocations
}

/// What this thread allocated while `f` ran: allocations, bytes, and the
/// most bytes held at once above those held when `f` started.
///
/// `f`'s result is dropped after the counts are read, so memory it returns
/// is held at the peak. A `measure` inside `f` leaves the outer peak
/// correct. Allocations on other threads are not counted, and every count is
/// 0 unless [`CountingAllocator`] is the global allocator.
pub fn measure<R>(f: impl FnOnce() -> R) -> Usage {
    let allocations = get(&ALLOCATIONS);
    let bytes = get(&BYTES);
    let live = get(&LIVE);
    let outer_peak = PEAK.try_with(|peak| peak.replace(live)).unwrap_or(0);
    let result = f();
    let peak = get(&PEAK);
    let usage = Usage {
        allocations: get(&ALLOCATIONS).saturating_sub(allocations),
        bytes: get(&BYTES).saturating_sub(bytes),
        peak: peak.saturating_sub(live),
    };
    let _ = PEAK.try_with(|cell| cell.set(outer_peak.max(peak)));
    drop(result);
    usage
}

#[expect(
    unsafe_code,
    reason = "GlobalAlloc is an unsafe trait, so counting allocations needs unsafe code"
)]
// SAFETY: each method forwards its arguments unchanged to `System`, which
// meets the `GlobalAlloc` contract. The counting touches only thread-local
// integers that no allocation depends on. A call that returns null allocated
// nothing and is not counted.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: the caller upholds `alloc`'s contract for `layout`, which
        // passes to System unchanged.
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            record_alloc(layout.size());
        }
        ptr
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: the caller upholds `alloc_zeroed`'s contract for `layout`,
        // which passes to System unchanged.
        let ptr = unsafe { System.alloc_zeroed(layout) };
        if !ptr.is_null() {
            record_alloc(layout.size());
        }
        ptr
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: `ptr` was allocated by this allocator, so by System, with
        // `layout`, and the caller upholds `realloc`'s contract for
        // `new_size`.
        let moved = unsafe { System.realloc(ptr, layout, new_size) };
        if !moved.is_null() {
            add(&ALLOCATIONS, 1);
            add(&BYTES, new_size);
            shrink(layout.size());
            grow(new_size);
        }
        moved
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        shrink(layout.size());
        // SAFETY: `ptr` was allocated by this allocator, so by System, with
        // `layout`.
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::{CountingAllocator, Usage, count, measure};

    #[global_allocator]
    static ALLOCATOR: CountingAllocator = CountingAllocator;

    #[test]
    fn nothing_allocated_counts_zero() {
        let buffer: Vec<u8> = Vec::with_capacity(8);
        assert_eq!(count(|| black_box(buffer.capacity())), 0);
        assert_eq!(measure(|| black_box(buffer.capacity())), Usage::default());
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
    fn bytes_count_each_request_and_reallocations_at_their_new_size() {
        let usage = measure(|| {
            let mut buffer: Vec<u8> = Vec::with_capacity(16);
            buffer.reserve_exact(64);
            black_box(buffer)
        });
        assert_eq!(usage.allocations, 2);
        assert_eq!(usage.bytes, 16 + 64);
        assert_eq!(usage.peak, 64);
    }

    #[test]
    fn the_peak_is_the_most_held_at_once() {
        let usage = measure(|| {
            drop(black_box(vec![0_u8; 100]));
            black_box(vec![0_u8; 40])
        });
        assert_eq!(usage.bytes, 140);
        assert_eq!(usage.peak, 100);
    }

    #[test]
    fn the_peak_ignores_what_was_held_before() {
        let held = black_box(vec![0_u8; 1_000]);
        let usage = measure(|| black_box(vec![0_u8; 10]));
        assert_eq!(usage.peak, 10);
        drop(held);
    }

    #[test]
    fn an_inner_measure_leaves_the_outer_peak_alone() {
        let outer = measure(|| {
            drop(black_box(vec![0_u8; 200]));
            let inner = measure(|| black_box(vec![0_u8; 50]));
            assert_eq!(inner.peak, 50);
        });
        assert_eq!(outer.peak, 200);
        assert_eq!(outer.bytes, 250);
    }

    #[test]
    fn a_thread_counts_its_own_allocations() {
        let usage = std::thread::spawn(|| measure(|| black_box(vec![1_u8; 4])))
            .join()
            .unwrap();
        assert_eq!(usage.allocations, 1);
        assert_eq!(usage.bytes, 4);
    }
}
