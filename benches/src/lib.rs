//! see benches/

pub use chunked_deque::Deque;
pub use criterion::{AxisScale, Bencher, BenchmarkId, Criterion, PlotConfiguration, criterion_group, criterion_main};
pub use rand::{RngExt, rngs::ThreadRng, seq::SliceRandom};
use std::convert::identity;
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
                let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
                let name = |s: &'static str| {
                    let name = s.strip_suffix(share).unwrap().trim_end_matches('_');
                    if name.is_empty() { "_" } else { name }
                };
                let mut group = $i.benchmark_group(share);
                group.plot_config(plot_config);
                $d(
                    #[allow(unreachable_code)]
                    let _ = || $d id(loop {}, loop {});
                    for n in $crate::VEC_SIZES {
                        let id = BenchmarkId::new(name(stringify!($d id)), n);
                        group.bench_with_input(id, &n, $d id);
                    }
                )+
            }};
        }
    };
}
/// input_array![]
macro_rules! input_array { ($($e:expr),+) => { [$(Input($e)),+] }; }

#[cfg(feature = "multi")]
pub const VEC_SIZES: [Input; 5] = input_array![100, 1000, 10000, 100000, 1000000];
#[cfg(not(feature = "multi"))]
pub const VEC_SIZES: [Input; 1] = input_array![10000];

pub const VEC_SIZE: usize = 1024 * 10;
pub const MIN_CASE_ITERATION: usize = 100000;
pub const BATCH_SIZE: criterion::BatchSize = criterion::BatchSize::SmallInput;

#[derive(Debug, Clone, Copy)]
pub struct Input(usize);

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Input {
    pub fn size(&self) -> usize {
        self.0
    }

    fn data<T: DequeLike>(&self, rng: &mut ThreadRng, f: fn(u32) -> T::Elem) -> T {
        let mut deque = T::default();

        for _ in 0..VEC_SIZE {
            let elem = rng.random::<u32>();
            if &elem & 1 == 0 {
                deque.push_back(f(elem));
            } else {
                deque.push_front(f(elem));
            }
        }
        deque
    }

    pub fn vec_deque(&self, rng: &mut ThreadRng) -> VecDeque<u32> {
        self.data(rng, identity)
    }

    pub fn chunked_deque(&self, rng: &mut ThreadRng) -> Deque<u32> {
        self.data(rng, identity)
    }

    pub fn linked_list(&self, rng: &mut ThreadRng) -> LinkedList<u32> {
        self.data(rng, identity)
    }

    pub fn vec_deque_as<T>(&self, rng: &mut ThreadRng, f: fn(u32) -> T) -> VecDeque<T> {
        self.data(rng, f)
    }

    pub fn chunked_deque_as<T>(&self, rng: &mut ThreadRng, f: fn(u32) -> T) -> Deque<T> {
        self.data(rng, f)
    }
}

trait DequeLike: Default {
    type Elem;
    fn push_back(&mut self, value: Self::Elem);
    fn push_front(&mut self, value: Self::Elem);
}

impl<T> DequeLike for VecDeque<T> {
    type Elem = T;

    fn push_back(&mut self, value: Self::Elem) {
        self.push_back(value);
    }

    fn push_front(&mut self, value: Self::Elem) {
        self.push_front(value);
    }
}

impl<T> DequeLike for Deque<T> {
    type Elem = T;

    fn push_back(&mut self, value: Self::Elem) {
        self.push_back(value);
    }

    fn push_front(&mut self, value: Self::Elem) {
        self.push_front(value);
    }
}

impl<T> DequeLike for LinkedList<T> {
    type Elem = T;

    fn push_back(&mut self, value: Self::Elem) {
        self.push_back(value);
    }

    fn push_front(&mut self, value: Self::Elem) {
        self.push_front(value);
    }
}
