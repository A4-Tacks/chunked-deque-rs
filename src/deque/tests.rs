use std::rc::Rc;

use crate::deque::Deque;

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
}

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
