use alloc::collections::VecDeque;

use crate::chunk::{Chunk, InChunk, LayoutInfo, Size as _};

#[cfg(test)]
mod tests;

pub(crate) mod iter;

const _: () =
    assert!(size_of::<Option<Deque<i16>>>() == size_of::<Deque<i16>>());

pub struct Deque<T> {
    info: LayoutInfo,
    chunks: VecDeque<Chunk<T>>,
    right: InChunk,
    left: InChunk,
}

impl<T: core::fmt::Debug> core::fmt::Debug for Deque<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entry(&self.iter()).finish()
    }
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}

impl<T> Deque<T> {
    pub const fn new() -> Self {
        Self::with_layout(LayoutInfo::auto::<T>())
    }

    /// Manual set chunk size.
    ///
    /// # Panics
    ///
    /// Panics if `size == 0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chunked_deque::Deque;
    /// let deque = Deque::<i32>::with_chunksize(176);
    /// assert_eq!(deque.chunk_size(), 176);
    /// ```
    pub const fn with_chunksize(size: u16) -> Deque<T> {
        assert!(size != 0, "chunk size by zero");
        Self::with_layout(LayoutInfo::with(size))
    }

    const fn with_layout(info: LayoutInfo) -> Self {
        Self {
            info,
            chunks: VecDeque::new(),
            right: 0,
            left: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.info.count() * self.chunks.capacity()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        match self.chunks.len() {
            0 => 0,
            1 => (self.right - self.left + 1).size(),
            chunks => {
                let count = self.info.count();
                (chunks - 2) * count
                    + (count - self.left.size())
                    + self.right.size()
                    + 1
            }
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn chunk_size(&self) -> usize {
        self.info.count()
    }

    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn back(&self) -> Option<&T> {
        Some(unsafe { self.chunks.back()?.get(self.right.size()).as_ref() })
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        Some(unsafe {
            self.chunks.back_mut()?.get(self.right.size()).as_mut()
        })
    }

    pub fn push_back(&mut self, value: T) {
        if self.chunks.is_empty() || self.info.for_inc(&mut self.right) {
            self.chunks.push_back(Chunk::new(self.info));
        }
        unsafe {
            self.chunks
                .back_mut()
                .unwrap_unchecked()
                .write(self.right.size(), value);
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let value = unsafe { self.chunks.back_mut()?.take(self.right.size()) };
        if self.pop_is_empty() || self.info.for_dec(&mut self.right) {
            unsafe { self.chunks.pop_back().unwrap().dealloc(self.info) };
        }
        Some(value)
    }

    pub fn front(&self) -> Option<&T> {
        Some(unsafe { self.chunks.front()?.get(self.left.size()).as_ref() })
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        Some(unsafe {
            self.chunks.front_mut()?.get(self.left.size()).as_mut()
        })
    }

    pub fn push_front(&mut self, value: T) {
        if self.chunks.is_empty() || self.info.for_dec(&mut self.left) {
            self.chunks.push_front(Chunk::new(self.info));
        }
        unsafe {
            self.chunks
                .front_mut()
                .unwrap_unchecked()
                .write(self.left.size(), value);
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let value = unsafe { self.chunks.front_mut()?.take(self.left.size()) };
        if self.pop_is_empty() || self.info.for_inc(&mut self.left) {
            unsafe { self.chunks.pop_front().unwrap().dealloc(self.info) };
        }
        Some(value)
    }

    #[inline(always)]
    fn pop_is_empty(&self) -> bool {
        self.chunks.len() == 1 && self.left == self.right
    }

    pub fn iter(&self) -> iter::Iter<'_, T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> iter::IterMut<'_, T> {
        self.into_iter()
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new()
    }
}
