use alloc::collections::VecDeque;
use alloc::rc::Rc;
use alloc::string::{String, ToString};

use crate::deque::Deque;

extern crate std;
#[allow(unused_imports)]
use std::dbg;

struct DropCheck(Rc<String>);

impl Drop for DropCheck {
    fn drop(&mut self) {
        assert_eq!(Rc::strong_count(&self.0), 1);
    }
}

impl DropCheck {
    fn new() -> Self {
        DropCheck(Rc::new("test".into()))
    }

    fn value(&self) -> Rc<String> {
        self.0.clone()
    }

    fn pair(&self) -> Pair {
        (self.0.clone(), Rc::strong_count(&self.0))
    }
}
type Pair = (Rc<String>, usize);

#[test]
fn push_back_len() {
    let mut deque = Deque::new();

    for i in 0..1000 {
        assert_eq!(deque.len(), i);
        deque.push_back(i.to_string());
    }
}

#[test]
fn push_back_can_drop() {
    let droper = DropCheck::new();
    let mut deque = Deque::new();

    for _ in 0..1000 {
        deque.push_back(droper.value());
    }
}

#[test]
fn push_front_len() {
    let mut deque = Deque::new();

    for i in 0..3 {
        assert_eq!(deque.len(), i);
        deque.push_front(i.to_string());
    }
}

#[test]
fn push_front_can_drop() {
    let droper = DropCheck::new();
    let mut deque = Deque::new();

    for _ in 0..1000 {
        deque.push_front(droper.value());
    }
}

#[test]
fn mixed_push() {
    let mut rand = oorandom::Rand64::new(0);
    for _ in 0..3 {
        let mut deque = Deque::new();

        for i in 0..3000 {
            if rand.rand_u64() & 1 == 0 {
                deque.push_back(i);
            } else {
                deque.push_front(i);
            }
        }

        for _ in 0..800 {
            if rand.rand_u64() & 1 == 0 {
                deque.pop_back();
            } else {
                deque.pop_front();
            }
        }
    }
}

fn mixed_from(n: i32) -> Deque<i32> {
    let mut rand = oorandom::Rand64::new(7);
    let mut deque = Deque::new();
    let mid = (rand.rand_u64() as u32 % n as u32) as i32;
    let mut iter = 0..mid;
    while let Some(next_back) = iter.next_back() {
        deque.push_front(next_back);
    }
    deque.extend(mid..n);
    assert_eq!(deque.len(), n as usize);
    deque
}

#[test]
fn test_into_iter_len() {
    let mut rand = oorandom::Rand64::new(0);
    let mut deque = mixed_from(1000).into_iter();
    let mut output = VecDeque::new();

    for i in 0..1000 {
        assert_eq!(deque.len(), 1000 - i);
        if rand.rand_u64() & 1 == 0 {
            output.push_back(deque.next_back().unwrap());
        } else {
            output.push_front(deque.next().unwrap());
        }
    }
}

#[test]
fn test_into_iter() {
    let deque = mixed_from(1000).into_iter();

    assert!(deque.eq(0..1000));
}

#[test]
fn test_into_iter_rev() {
    let deque = mixed_from(1000).into_iter().rev();

    assert!(deque.eq((0..1000).rev()));
}

#[test]
fn test_into_iter_mix() {
    let mut rand = oorandom::Rand64::new(0);
    let mut deque = mixed_from(1000).into_iter();
    let mut vecdeque = VecDeque::from_iter(0..1000).into_iter();

    for _ in 0..1000 {
        if rand.rand_u64() & 1 == 0 {
            assert_eq!(deque.next_back(), vecdeque.next_back());
        } else {
            assert_eq!(deque.next(), vecdeque.next());
        }
    }
}

#[test]
fn test_iter_mix() {
    let mut rand = oorandom::Rand64::new(0);
    let deque = mixed_from(1000);
    let vecdeque = VecDeque::from_iter(0..1000);
    let (mut deque, mut vecdeque) = (deque.iter(), vecdeque.iter());

    for _ in 0..1000 {
        if rand.rand_u64() & 1 == 0 {
            assert_eq!(deque.next_back(), vecdeque.next_back());
        } else {
            assert_eq!(deque.next(), vecdeque.next());
        }
    }
}

#[derive(Debug)]
enum Action {
    PushBack,
    PushFront,
    PopBack,
    PopFront,
    Len,
    Front,
    Back,
    Fill,
}

impl Action {
    fn from_num(n: u64) -> Action {
        match n % 8 {
            0 => Self::PushBack,
            1 => Self::PushFront,
            2 => Self::PopBack,
            3 => Self::PopFront,
            4 => Self::Len,
            5 => Self::Front,
            6 => Self::Back,
            7 => Self::Fill,
            _ => unreachable!(),
        }
    }

    fn do_both(
        self,
        state: &DropCheck,
        a: &mut Deque<Pair>,
        b: &mut VecDeque<Pair>,
    ) {
        match self {
            Action::PushBack => {
                let value = state.pair();
                a.push_back(value.clone());
                b.push_back(value);
            }
            Action::PushFront => {
                let value = state.pair();
                a.push_front(value.clone());
                b.push_front(value);
            }
            Action::PopBack => assert_eq!(a.pop_back(), b.pop_back()),
            Action::PopFront => assert_eq!(a.pop_front(), b.pop_front()),
            Action::Len => assert_eq!(a.len(), b.len()),
            Action::Front => {
                assert_eq!(a.front(), b.front());
                assert_eq!(a.front_mut(), b.front_mut());
            }
            Action::Back => {
                assert_eq!(a.back(), b.back());
                assert_eq!(a.back_mut(), b.back_mut());
            }
            Action::Fill => {
                let value = state.pair();
                a.push_front(value.clone());
                b.push_front(value);
                let value = state.pair();
                a.push_back(value.clone());
                b.push_back(value);
            }
        }
    }
}

#[test]
fn fuzzy() {
    let drop_check = DropCheck::new();
    let mut rand = oorandom::Rand64::new(0);
    let mut a = Deque::new();
    let mut b = VecDeque::new();

    for _ in 0..10000 {
        let action = Action::from_num(rand.rand_u64());
        action.do_both(&drop_check, &mut a, &mut b);
    }
}
