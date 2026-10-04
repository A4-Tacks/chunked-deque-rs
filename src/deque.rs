use std::collections::VecDeque;

use crate::chunk::{Chunk, InChunk, LayoutInfo, Size as _};

#[cfg(test)]
mod tests;

const _: () =
    assert!(size_of::<Option<Deque<i16>>>() == size_of::<Deque<i16>>());

// |<-------|-----------------|----------->| chunks * info
//   |<---->|<--------------->|<----->|
//     left         mid         right
//                  ^^^ chunks * info - (left != 0) - (right != 0)?
//                  我得想想这个模型
//
// ==0时不存在?
// 还是left right总是inclusive吧, 双指针逻辑

pub struct Deque<T> {
    info: LayoutInfo,
    chunks: VecDeque<Chunk<T>>,
    right: InChunk,
    left: InChunk,
}

impl<T> std::fmt::Debug for Deque<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Deque")
            .field("info", &self.info)
            .field("left", &self.left)
            .field("chunks", &self.chunks)
            .field("right", &self.right)
            .finish()
    }
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        let info = Chunk::<T>::layout();
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

    pub fn back(&self) -> Option<&T> {
        Some(unsafe { self.chunks.back()?.get(self.right.size()).as_ref() })
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
}
