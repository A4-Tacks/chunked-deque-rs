use core::ptr::NonNull;

pub(crate) use crate::chunk::layout::LayoutInfo;
pub(crate) type InChunk = u16;

const _: () = assert!(size_of::<InChunk>() <= size_of::<usize>());

pub trait Size {
    fn size(self) -> usize;
}

impl Size for InChunk {
    #[inline]
    fn size(self) -> usize {
        self as usize
    }
}

/// This value from `_GLIBCXX_DEQUE_BUF_SIZE`
const _DEQUE_CHUNK_SIZE: usize = 512;

mod layout {
    use alloc::alloc::Layout;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct LayoutInfo(pub(super) u16);

    impl LayoutInfo {
        pub(super) fn get<T>(self) -> Layout {
            Layout::array::<T>(self.0.try_into().unwrap()).unwrap()
        }

        pub fn count(self) -> usize {
            self.0.into()
        }
    }
}

impl LayoutInfo {
    pub fn for_inc(self, side: &mut InChunk) -> bool {
        debug_assert_ne!(self.count(), 0);
        debug_assert!(side.size() <= self.count(), "{side}");

        *side += 1;

        if *side == self.count() as InChunk {
            *side = 0;
            true
        } else {
            false
        }
    }

    pub fn for_dec(self, side: &mut InChunk) -> bool {
        debug_assert_ne!(self.count(), 0);
        debug_assert!(side.size() <= self.count(), "{side}");

        if *side == 0 {
            *side = self.count() as InChunk - 1;
            true
        } else {
            *side -= 1;
            false
        }
    }
}

pub(crate) struct Chunk<T> {
    data: NonNull<T>,
    #[cfg(test)]
    slots: core::cell::RefCell<alloc::vec::Vec<bool>>,
}

impl<T> core::fmt::Debug for Chunk<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Chunk")?;
        #[cfg(test)]
        write!(f, "{:?}", self.slots.borrow())?;
        Ok(())
    }
}

impl<T> Chunk<T> {
    pub fn layout() -> LayoutInfo {
        LayoutInfo(4)
    }

    #[allow(unused_variables)]
    #[inline(always)]
    #[track_caller]
    fn expect(&self, expect: bool, index: usize) {
        #[cfg(test)]
        {
            assert_eq!(self.slots.borrow()[index], expect, "index: {index}");
        }
    }

    #[allow(unused_variables)]
    #[inline(always)]
    #[track_caller]
    fn flip_expect(&self, expect: bool, index: usize) {
        #[cfg(test)]
        {
            self.expect(expect, index);
            self.slots.borrow_mut()[index] = !expect;
        }
    }

    pub fn new(info: LayoutInfo) -> Self {
        let data = unsafe { alloc::alloc::alloc(info.get::<T>()) };
        let data = NonNull::new(data).expect("can't alloc chunk").cast();

        Self {
            data,
            #[cfg(test)]
            slots: core::cell::RefCell::new(alloc::vec![false; info.count()]),
        }
    }

    #[track_caller]
    pub unsafe fn get(&self, index: usize) -> NonNull<T> {
        self.expect(true, index);
        unsafe { self.data.add(index) }
    }

    #[track_caller]
    pub unsafe fn iter(
        &self,
        start: usize,
        end: usize,
    ) -> core::slice::Iter<'_, T> {
        if start < end {
            self.expect(true, start);
            self.expect(true, end - 1);
            unsafe {
                let data = self.data.add(start);
                core::ptr::slice_from_raw_parts(
                    data.as_ptr().cast_const(),
                    end - start,
                )
                .as_ref_unchecked()
                .iter()
            }
        } else {
            core::slice::Iter::default()
        }
    }

    #[track_caller]
    pub unsafe fn iter_mut(
        &self,
        start: usize,
        end: usize,
    ) -> core::slice::IterMut<'_, T> {
        if start < end {
            self.expect(true, start);
            self.expect(true, end - 1);
            unsafe {
                let data = self.data.add(start);
                core::ptr::slice_from_raw_parts_mut(data.as_ptr(), end - start)
                    .as_mut_unchecked()
                    .iter_mut()
            }
        } else {
            core::slice::IterMut::default()
        }
    }

    #[track_caller]
    pub fn write(&mut self, index: usize, value: T) {
        self.flip_expect(false, index);
        unsafe { self.get(index).write(value) };
    }

    #[track_caller]
    pub unsafe fn take(&mut self, index: usize) -> T {
        self.expect(true, index);
        unsafe {
            let get = self.get(index);
            self.flip_expect(true, index);
            get.read()
        }
    }

    #[track_caller]
    pub unsafe fn dealloc(self, info: LayoutInfo) {
        #[cfg(test)]
        for index in 0..self.slots.borrow().len() {
            self.expect(false, index);
        }
        unsafe {
            alloc::alloc::dealloc(self.data.as_ptr().cast(), info.get::<T>())
        };
    }
}
