//! A list of at most `N` items kept inline, for the small sets of
//! alternatives a matcher tries, so that trying them allocates nothing.

use std::ops::Deref;

/// At most `N` items, in the order they were pushed.
#[derive(Clone, Copy)]
pub(crate) struct Few<T: Copy, const N: usize> {
    items: [T; N],
    len: usize,
}

impl<T: Copy + Default, const N: usize> Few<T, N> {
    /// No items.
    pub(crate) fn new() -> Self {
        Few {
            items: [T::default(); N],
            len: 0,
        }
    }
}

impl<T: Copy, const N: usize> Few<T, N> {
    /// Adds `item` at the end. Each list is sized for every item its
    /// expression can give, so an item past `N` cannot come; it would be
    /// dropped.
    pub(crate) fn push(&mut self, item: T) {
        if let Some(slot) = self.items.get_mut(self.len) {
            *slot = item;
            self.len += 1;
        }
    }

    /// Whether the list holds `N` items.
    pub(crate) fn is_full(&self) -> bool {
        self.len == N
    }
}

impl<T: Copy + Default, const N: usize> FromIterator<T> for Few<T, N> {
    /// The first `N` items of `iter`.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut few = Few::new();
        for item in iter.into_iter().take(N) {
            few.push(item);
        }
        few
    }
}

impl<T: Copy, const N: usize> Deref for Few<T, N> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.items[..self.len]
    }
}

impl<T: Copy, const N: usize> IntoIterator for Few<T, N> {
    type Item = T;
    type IntoIter = std::iter::Take<std::array::IntoIter<T, N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter().take(self.len)
    }
}
