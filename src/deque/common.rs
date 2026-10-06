use core::hash::Hash;

use alloc::vec::Vec;

use crate::Deque;

impl<T: Clone> Clone for Deque<T> {
    fn clone(&self) -> Self {
        // XXX: perf
        let mut deque =
            Deque::with_chunksize(self.chunk_size().try_into().unwrap());
        deque.extend(self.iter().cloned());
        deque
    }
}

impl<T: Eq> Eq for Deque<T> {}

macro_rules! partial_cmp {
    ($ty:ty $(, $($t:tt)*)?) => {
        impl<T: PartialEq $(, $($t)*)?> PartialEq<$ty> for Deque<T> {
            fn eq(&self, other: &$ty) -> bool {
                self.iter().eq(other.iter())
            }
        }
    };
}

partial_cmp!(Deque<T>);
partial_cmp!(Vec<T>);
partial_cmp!([T]);
partial_cmp!(&[T]);
partial_cmp!(&mut [T]);
partial_cmp!([T; N], const N: usize);
partial_cmp!(&[T; N], const N: usize);
partial_cmp!(&mut [T; N], const N: usize);

impl<T: PartialOrd> PartialOrd for Deque<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for Deque<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.iter().cmp(other)
    }
}

impl<T: Hash> Hash for Deque<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.iter().for_each(|ele| ele.hash(state));
    }
}

// NOTE: 暂时不实现 Extend<&(T: Copy)>, 因为这会造成'可能很快'的暗示
// 等普通的extend和clone改进后再说, Read Write 同理
