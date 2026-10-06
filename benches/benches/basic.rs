use std::{collections::VecDeque, hint::black_box};

use chunked_deque::Deque;
use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use rand::{RngExt, rngs::ThreadRng, seq::SliceRandom};

const VEC_SIZE: usize = 1024 * 10;
const MIN_CASE_ITERATION: usize = 100000;
const BATCH_SIZE: criterion::BatchSize = criterion::BatchSize::SmallInput;

fn random_vec_deque(rng: &mut ThreadRng) -> VecDeque<u32> {
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

fn random_chunked_deque(rng: &mut ThreadRng) -> Deque<u32> {
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

    b.iter_batched_ref(
        || random_vec_deque(&mut rng),
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

    b.iter_batched_ref(
        || random_chunked_deque(&mut rng),
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

    b.iter_batched_ref(
        || random_vec_deque(&mut rng),
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

    b.iter_batched_ref(
        || random_chunked_deque(&mut rng),
        |deque| {
            while !deque.is_empty() {
                black_box(deque.pop_front());
            }
        },
        BATCH_SIZE,
    );
}

fn vec_deque_get(b: &mut Bencher) {
    let mut rng = rand::rng();
    let mut query = vec![];

    for _ in 0..VEC_SIZE / 2 {
        query.push(rng.random::<u64>() as usize % VEC_SIZE);
    }
    b.iter_batched_ref(
        || {
            query.shuffle(&mut rng);
            (random_vec_deque(&mut rng), query.clone())
        },
        |(deque, query)| {
            for &mut index in query {
                black_box(deque.get(index));
            }
        },
        BATCH_SIZE,
    );
}

fn chunked_deque_get(b: &mut Bencher) {
    let mut rng = rand::rng();
    let mut query = vec![];

    for _ in 0..VEC_SIZE / 2 {
        query.push(rng.random::<u64>() as usize % VEC_SIZE);
    }
    b.iter_batched_ref(
        || {
            query.shuffle(&mut rng);
            (random_chunked_deque(&mut rng), query.clone())
        },
        |(deque, query)| {
            for &mut index in query {
                black_box(deque.get(index));
            }
        },
        BATCH_SIZE,
    );
}

fn vec_deque_read_front(b: &mut Bencher) {
    let mut rng = rand::rng();

    b.iter_batched_ref(
        || random_vec_deque(&mut rng),
        |deque| {
            for _ in 0..MIN_CASE_ITERATION {
                black_box(deque.front());
            }
        },
        BATCH_SIZE,
    );
}

fn chunked_deque_read_front(b: &mut Bencher) {
    let mut rng = rand::rng();

    b.iter_batched_ref(
        || random_chunked_deque(&mut rng),
        |deque| {
            for _ in 0..MIN_CASE_ITERATION {
                black_box(deque.front());
            }
        },
        BATCH_SIZE,
    );
}

fn vec_deque_read_back(b: &mut Bencher) {
    let mut rng = rand::rng();

    b.iter_batched_ref(
        || random_vec_deque(&mut rng),
        |deque| {
            for _ in 0..MIN_CASE_ITERATION {
                black_box(deque.front());
            }
        },
        BATCH_SIZE,
    );
}

fn chunked_deque_read_back(b: &mut Bencher) {
    let mut rng = rand::rng();

    b.iter_batched_ref(
        || random_chunked_deque(&mut rng),
        |deque| {
            for _ in 0..MIN_CASE_ITERATION {
                black_box(deque.front());
            }
        },
        BATCH_SIZE,
    );
}

fn vec_deque_iterate(b: &mut Bencher) {
    let mut rng = rand::rng();

    b.iter_batched_ref(
        || random_vec_deque(&mut rng),
        |deque| {
            for elem in deque {
                black_box(elem);
            }
        },
        BATCH_SIZE,
    );
}

fn chunked_deque_iterate(b: &mut Bencher) {
    let mut rng = rand::rng();

    b.iter_batched_ref(
        || random_chunked_deque(&mut rng),
        |deque| {
            for elem in deque {
                black_box(elem);
            }
        },
        BATCH_SIZE,
    );
}

fn vec_deque_iterate_rev(b: &mut Bencher) {
    let mut rng = rand::rng();

    b.iter_batched_ref(
        || random_vec_deque(&mut rng),
        |deque| {
            for elem in deque.iter().rev() {
                black_box(elem);
            }
        },
        BATCH_SIZE,
    );
}

fn chunked_deque_iterate_rev(b: &mut Bencher) {
    let mut rng = rand::rng();

    b.iter_batched_ref(
        || random_chunked_deque(&mut rng),
        |deque| {
            for elem in deque.iter().rev() {
                black_box(elem);
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
    bench!(vec_deque_get, chunked_deque_get);
    bench!(vec_deque_read_front, chunked_deque_read_front);
    bench!(vec_deque_read_back, chunked_deque_read_back);
    bench!(vec_deque_iterate, chunked_deque_iterate);
    bench!(vec_deque_iterate_rev, chunked_deque_iterate_rev);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
