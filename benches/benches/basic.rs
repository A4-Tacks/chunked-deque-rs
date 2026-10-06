use std::{collections::VecDeque, hint::black_box};

use chunked_deque::Deque;
use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use rand::RngExt;

const VEC_SIZE: usize = 1024 * 10;
const BATCH_SIZE: criterion::BatchSize = criterion::BatchSize::SmallInput;

fn vec_deque_push_back(b: &mut Bencher) {
    let mut rng = rand::rng();
    let mut datas = vec![];
    let deque = VecDeque::new();

    for _ in 0..VEC_SIZE {
        datas.push(rng.random::<u32>());
    }
    b.iter_batched_ref(
        || deque.clone(),
        |deque| {
            for &elem in &datas {
                deque.push_back(elem);
            }
        },
        BATCH_SIZE,
    );
}

fn chunked_deque_push_back(b: &mut Bencher) {
    let mut rng = rand::rng();
    let mut datas = vec![];
    let deque = Deque::new();

    for _ in 0..VEC_SIZE {
        datas.push(rng.random::<u32>());
    }
    b.iter_batched_ref(
        || deque.clone(),
        |deque| {
            for &elem in &datas {
                deque.push_back(elem);
            }
        },
        BATCH_SIZE,
    );
}

fn vec_deque_push_front(b: &mut Bencher) {
    let mut rng = rand::rng();
    let mut datas = vec![];
    let deque = VecDeque::new();

    for _ in 0..VEC_SIZE {
        datas.push(rng.random::<u32>());
    }
    b.iter_batched_ref(
        || deque.clone(),
        |deque| {
            for &elem in &datas {
                deque.push_front(elem);
            }
        },
        BATCH_SIZE,
    );
}

fn chunked_deque_push_front(b: &mut Bencher) {
    let mut rng = rand::rng();
    let mut datas = vec![];
    let deque = Deque::new();

    for _ in 0..VEC_SIZE {
        datas.push(rng.random::<u32>());
    }
    b.iter_batched_ref(
        || deque.clone(),
        |deque| {
            for &elem in &datas {
                deque.push_front(elem);
            }
        },
        BATCH_SIZE,
    );
}

fn vec_deque_push_mixed(b: &mut Bencher) {
    let mut rng = rand::rng();
    let mut datas = vec![];
    let deque = VecDeque::new();

    for _ in 0..VEC_SIZE {
        datas.push(rng.random::<u32>());
    }
    b.iter_batched_ref(
        || deque.clone(),
        |deque| {
            for &elem in &datas {
                if &elem & 1 == 0 {
                    deque.push_back(elem);
                } else {
                    deque.push_front(elem);
                }
            }
        },
        BATCH_SIZE,
    );
}

fn chunked_deque_push_mixed(b: &mut Bencher) {
    let mut rng = rand::rng();
    let mut datas = vec![];
    let deque = Deque::new();

    for _ in 0..VEC_SIZE {
        datas.push(rng.random::<u32>());
    }
    b.iter_batched_ref(
        || deque.clone(),
        |deque| {
            for &elem in &datas {
                if &elem & 1 == 0 {
                    deque.push_back(elem);
                } else {
                    deque.push_front(elem);
                }
            }
        },
        BATCH_SIZE,
    );
}

fn vec_deque_pop_back(b: &mut Bencher) {
    let mut rng = rand::rng();
    let mut deque = VecDeque::new();

    for _ in 0..VEC_SIZE {
        let elem = rng.random::<u32>();
        if &elem & 1 == 0 {
            deque.push_back(elem);
        } else {
            deque.push_front(elem);
        }
    }
    b.iter_batched_ref(
        || deque.clone(),
        |deque| {
            while !deque.is_empty() {
                black_box(deque.pop_back());
            }
        },
        BATCH_SIZE,
    );
}

fn chunked_deque_pop_back(b: &mut Bencher) {
    let mut rng = rand::rng();
    let mut deque = Deque::new();

    for _ in 0..VEC_SIZE {
        let elem = rng.random::<u32>();
        if &elem & 1 == 0 {
            deque.push_back(elem);
        } else {
            deque.push_front(elem);
        }
    }
    b.iter_batched_ref(
        || deque.clone(),
        |deque| {
            while !deque.is_empty() {
                black_box(deque.pop_back());
            }
        },
        BATCH_SIZE,
    );
}

fn vec_deque_pop_front(b: &mut Bencher) {
    let mut rng = rand::rng();
    let mut deque = VecDeque::new();

    for _ in 0..VEC_SIZE {
        let elem = rng.random::<u32>();
        if &elem & 1 == 0 {
            deque.push_back(elem);
        } else {
            deque.push_front(elem);
        }
    }
    b.iter_batched_ref(
        || deque.clone(),
        |deque| {
            while !deque.is_empty() {
                black_box(deque.pop_front());
            }
        },
        BATCH_SIZE,
    );
}

fn chunked_deque_pop_front(b: &mut Bencher) {
    let mut rng = rand::rng();
    let mut deque = Deque::new();

    for _ in 0..VEC_SIZE {
        let elem = rng.random::<u32>();
        if &elem & 1 == 0 {
            deque.push_back(elem);
        } else {
            deque.push_front(elem);
        }
    }
    b.iter_batched_ref(
        || deque.clone(),
        |deque| {
            while !deque.is_empty() {
                black_box(deque.pop_front());
            }
        },
        BATCH_SIZE,
    );
}

fn share_suffix<'a>(s: &[&'a str]) -> &'a str {
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

fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! bench {
        ($($id:ident),+) => {{
            let mut group = c.benchmark_group(share_suffix(&[$(stringify!($id)),+]));
            $(
                #[allow(unreachable_code)]
                let _ = || $id(loop {});
                group.bench_function(stringify!($id), $id);
            )+
        }};
    }

    bench!(vec_deque_push_back, chunked_deque_push_back);
    bench!(vec_deque_push_front, chunked_deque_push_front);
    bench!(vec_deque_push_mixed, chunked_deque_push_mixed);
    bench!(vec_deque_pop_back, chunked_deque_pop_back);
    bench!(vec_deque_pop_front, chunked_deque_pop_front);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
