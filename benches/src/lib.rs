//! see benches/

pub use chunked_deque::Deque;
pub use criterion::{Bencher, Criterion, criterion_group, criterion_main};
pub use rand::{RngExt, rngs::ThreadRng, seq::SliceRandom};
pub use std::{
    collections::{LinkedList, VecDeque},
    hint::black_box,
};

pub fn share_suffix<'a>(s: &[&'a str]) -> &'a str {
    let mut first = *s.first().unwrap();
    for s in s.iter().skip(1) {
        while !first.is_empty() && !s.ends_with(first) {
            first = &first[1..];
        }
    }
    first = first.trim_start_matches('_');
    assert_ne!(first, "");
    first
}

#[rustfmt::skip]
#[macro_export]
macro_rules! bench_caller {
    ($i:ident($d:tt)) => {
        macro_rules! bench {
            ($d($d id:ident),+) => {{
                let share = $crate::share_suffix(&[$d(stringify!($d id)),+]);
                let name = |s: &'static str| {
                    let name = s.strip_suffix(share).unwrap().trim_end_matches('_');
                    if name.is_empty() { "_" } else { name }
                };
                let mut group = $i.benchmark_group(share);
                $d(
                    #[allow(unreachable_code)]
                    let _ = || $d id(loop {});
                    group.bench_function(name(stringify!($d id)), $d id);
                )+
            }};
        }
    };
}

pub const VEC_SIZE: usize = 1024 * 10;
pub const MIN_CASE_ITERATION: usize = 100000;
pub const BATCH_SIZE: criterion::BatchSize = criterion::BatchSize::SmallInput;

pub fn random_vec_deque(rng: &mut ThreadRng) -> VecDeque<u32> {
    let mut deque = VecDeque::new();

    for _ in 0..VEC_SIZE {
        let elem = rng.random::<u32>();
        if &elem & 1 == 0 {
            deque.push_back(elem);
        } else {
            deque.push_front(elem);
        }
    }
    deque
}

pub fn random_chunked_deque(rng: &mut ThreadRng) -> Deque<u32> {
    let mut deque = Deque::new();

    for _ in 0..VEC_SIZE {
        let elem = rng.random::<u32>();
        if &elem & 1 == 0 {
            deque.push_back(elem);
        } else {
            deque.push_front(elem);
        }
    }
    deque
}
