use alloc::collections::VecDeque;

use crate::chunk::{Chunk, InChunk, LayoutInfo, Size as _};

#[cfg(test)]
mod tests;

mod common;
pub(crate) mod iter;

const _: () =
    assert!(size_of::<Option<Deque<i16>>>() == size_of::<Deque<i16>>());

/// Deque, similar `VecDeque<Box<[T]>>`, timely release of reserves for pop methods
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
        // FIXME: 如果只有一个值panic了, drop应该继续进行
        while self.pop_back().is_some() {}
    }
}

impl<T> Deque<T> {
    /// Create a [`Deque`].
    pub const fn new() -> Self {
        Self::with_layout(LayoutInfo::auto::<T>())
    }

    /// Manual set chunk size (the number of elements stored).
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

    /// Reserved capacity of elements.
    ///
    /// NOTE: Due to the two-level buffer,
    /// having sufficient capacity means not re-allocating `VecDeque<Chunk>`, not `Chunk`
    pub fn capacity(&self) -> usize {
        self.info.count() * self.chunks.capacity()
    }

    /// Elements count.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chunked_deque::Deque;
    /// let mut deque = Deque::new();
    /// assert_eq!(deque.len(), 0);
    /// deque.push_back(0);
    /// assert_eq!(deque.len(), 1);
    /// deque.push_front(0);
    /// assert_eq!(deque.len(), 2);
    /// ```
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

    /// Chunk size of inner `VecDeque<Chunk>` (the number of elements stored).
    #[inline]
    pub fn chunk_size(&self) -> usize {
        self.info.count()
    }

    /// Chunk count of inner `VecDeque<Chunk>`.
    #[inline]
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    /// Chunk capacity of inner `VecDeque<Chunk>`.
    #[inline]
    pub fn chunk_capacity(&self) -> usize {
        self.chunks.capacity()
    }

    /// Last element.
    pub fn back(&self) -> Option<&T> {
        Some(unsafe { self.chunks.back()?.get(self.right.size()).as_ref() })
    }

    /// Last element with mutable reference.
    pub fn back_mut(&mut self) -> Option<&mut T> {
        Some(unsafe {
            self.chunks.back_mut()?.get(self.right.size()).as_mut()
        })
    }

    /// Push to last element.
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

    /// Pop off last element.
    pub fn pop_back(&mut self) -> Option<T> {
        let value = unsafe { self.chunks.back_mut()?.take(self.right.size()) };
        if self.pop_is_empty() || self.info.for_dec(&mut self.right) {
            unsafe {
                self.chunks.pop_back().unwrap_unchecked().dealloc(self.info)
            };
        }
        Some(value)
    }

    /// First element with.
    pub fn front(&self) -> Option<&T> {
        Some(unsafe { self.chunks.front()?.get(self.left.size()).as_ref() })
    }

    /// First element with mutable reference.
    pub fn front_mut(&mut self) -> Option<&mut T> {
        Some(unsafe {
            self.chunks.front_mut()?.get(self.left.size()).as_mut()
        })
    }

    /// Push to first element.
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

    /// Pop off first element.
    pub fn pop_front(&mut self) -> Option<T> {
        let value = unsafe { self.chunks.front_mut()?.take(self.left.size()) };
        if self.pop_is_empty() || self.info.for_inc(&mut self.left) {
            unsafe {
                self.chunks.pop_front().unwrap_unchecked().dealloc(self.info)
            };
        }
        Some(value)
    }

    #[inline(always)]
    fn pop_is_empty(&self) -> bool {
        self.chunks.len() == 1 && self.left == self.right
    }

    /// Iterate all elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chunked_deque::Deque;
    /// let mut deque = Deque::new();
    /// deque.push_back(2);
    /// deque.push_front(3);
    /// let elements = deque.iter().collect::<Vec<_>>();
    /// assert_eq!(elements, vec![&3, &2]);
    /// ```
    pub fn iter(&self) -> iter::Iter<'_, T> {
        self.into_iter()
    }

    /// Iterate all elements with mutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chunked_deque::Deque;
    /// let mut deque = Deque::new();
    /// deque.push_back(2);
    /// deque.push_front(3);
    /// let elements = deque.iter().collect::<Vec<_>>();
    /// assert_eq!(elements, vec![&mut 3, &mut 2]);
    /// ```
    pub fn iter_mut(&mut self) -> iter::IterMut<'_, T> {
        self.into_iter()
    }

    /// Get element of index.
    ///
    /// This may be a bit slow because of division and remainder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chunked_deque::Deque;
    /// let mut deque = Deque::new();
    /// deque.push_back(2);
    /// assert_eq!(deque.get(0), Some(&2));
    /// deque.push_front(3);
    /// assert_eq!(deque.get(0), Some(&3));
    /// assert_eq!(deque.get(1), Some(&2));
    /// assert_eq!(deque.get(0), deque.front());
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }
        // XXX: 也许未来可以试着为指数大小做一个特殊case看看性能?
        let chunk_size = self.chunk_size();
        let offset = index + self.left.size();
        unsafe {
            Some(
                self.chunks[offset / chunk_size]
                    .get(offset % chunk_size)
                    .as_ref(),
            )
        }
    }

    /// Get element of index with mutable reference.
    ///
    /// This may be a bit slow because of division and remainder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chunked_deque::Deque;
    /// let mut deque = Deque::new();
    /// deque.push_back(2);
    /// assert_eq!(deque.get_mut(0), Some(&mut 2));
    /// deque.push_front(3);
    /// assert_eq!(deque.get_mut(0), Some(&mut 3));
    /// assert_eq!(deque.get_mut(1), Some(&mut 2));
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len() {
            return None;
        }
        let chunk_size = self.chunk_size();
        let offset = index + self.left.size();
        unsafe {
            Some(
                self.chunks[offset / chunk_size]
                    .get(offset % chunk_size)
                    .as_mut(),
            )
        }
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new()
    }
}
