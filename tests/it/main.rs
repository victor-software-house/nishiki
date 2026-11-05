//! nishiki's integration tests, linked as one binary.

mod alloc;
mod fuzz;
mod limits;
mod notices;
mod oracle;
mod sorted;
mod stream;

/// A highlighter can move between threads, be shared, and survive a caught
/// panic. A cell in its scratch would take away `Sync` and `RefUnwindSafe`,
/// which breaks the public API.
#[test]
fn a_highlighter_is_send_sync_and_unwind_safe() {
    fn auto_traits<T: Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe>() {}
    auto_traits::<nishiki::Highlighter>();
}
