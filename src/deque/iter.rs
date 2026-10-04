use core::{iter::FusedIterator, slice};

use alloc::collections::vec_deque;

use crate::{
    Deque,
    chunk::{Chunk, Size},
};

impl<T> FromIterator<T> for Deque<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut deque = Deque::new();
        deque.extend(iter);
        deque
    }
}
impl<T> Extend<T> for Deque<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        iter.into_iter().for_each(|elem| self.push_back(elem));
    }
}

impl<T> IntoIterator for Deque<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            deque: self,
        }
    }
}

pub struct IntoIter<T> {
    deque: Deque<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.deque.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.deque.len(), Some(self.deque.len()))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.deque.pop_back()
    }
}

impl<T> FusedIterator for IntoIter<T> {}
impl<T> ExactSizeIterator for IntoIter<T> {}

macro_rules! impl_iter {
    ($Iter:ident, $iter:ident, $($mut:ident)?) => {
        impl<'a, T> IntoIterator for &'a $($mut)? Deque<T> {
            type Item = &'a $($mut)? T;
            type IntoIter = $Iter<'a, T>;

            fn into_iter(self) -> Self::IntoIter {
                $Iter::new(self)
            }
        }

        #[derive(Debug)]
        pub struct $Iter<'a, T: 'a> {
            iter: vec_deque::$Iter<'a, Chunk<T>>,
            front: slice::$Iter<'a, T>,
            back: slice::$Iter<'a, T>,
            chunk_size: usize,
        }

        impl<'a, T: 'a> Iterator for $Iter<'a, T> {
            type Item = &'a $($mut)? T;

            fn next(&mut self) -> Option<Self::Item> {
                match self.front.next() {
                    Some(val) => Some(val),
                    None => match self.or_front().and_then(|it| it.next()) {
                        Some(val) => Some(val),
                        None => self.back.next(),
                    },
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.front.len() + self.iter.len() + self.back.len();
                (len, Some(len))
            }
        }

        impl<'a, T: 'a> DoubleEndedIterator for $Iter<'a, T> {
            fn next_back(&mut self) -> Option<Self::Item> {
                match self.back.next_back() {
                    Some(val) => Some(val),
                    None => match self.or_back().and_then(|it| it.next_back())
                    {
                        Some(val) => Some(val),
                        None => self.front.next_back(),
                    },
                }
            }
        }
        impl<'a, T: 'a> FusedIterator for $Iter<'a, T> {}
        impl<'a, T: 'a> ExactSizeIterator for $Iter<'a, T> {}

        impl<'a, T: 'a> $Iter<'a, T> {
            pub(crate) fn new(deque: &'a $($mut)? Deque<T>) -> Self {
                let mut iter = deque.chunks.$iter();
                let chunk_size = deque.info.count();
                let front = iter.next().map_or_default(|chunk| unsafe {
                    chunk.$iter(deque.left.size(), chunk_size)
                });
                let back = iter.next_back().map_or_default(|chunk| unsafe {
                    chunk.$iter(0, deque.right.size() + 1)
                });
                Self {
                    iter,
                    front,
                    back,
                    chunk_size,
                }
            }

            fn or_front(&mut self) -> Option<&mut slice::$Iter<'a, T>> {
                self.front =
                    unsafe { self.iter.next()?.$iter(0, self.chunk_size) };
                Some(&mut self.front)
            }

            fn or_back(&mut self) -> Option<&mut slice::$Iter<'a, T>> {
                self.back =
                    unsafe { self.iter.next_back()?.$iter(0, self.chunk_size) };
                Some(&mut self.back)
            }
        }
    };
}

impl_iter!(Iter, iter,);
impl_iter!(IterMut, iter_mut, mut);

impl<'a, T: 'a> Clone for Iter<'a, T> {
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            front: self.front.clone(),
            back: self.back.clone(),
            chunk_size: self.chunk_size,
        }
    }
}
