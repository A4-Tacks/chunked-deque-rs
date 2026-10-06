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

fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! bench {
        ($id:ident) => {
            #[allow(unreachable_code)]
            let _ = || $id(loop {});
            c.bench_function(stringify!($id), $id);
        };
    }

    bench!(vec_deque_push_back);
    bench!(chunked_deque_push_back);

    bench!(vec_deque_push_front);
    bench!(chunked_deque_push_front);

    bench!(vec_deque_push_mixed);
    bench!(chunked_deque_push_mixed);

    bench!(vec_deque_pop_back);
    bench!(chunked_deque_pop_back);

    bench!(vec_deque_pop_front);
    bench!(chunked_deque_pop_front);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
