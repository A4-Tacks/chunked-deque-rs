use benches::*;

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

fn criterion_benchmark(c: &mut Criterion) {
    bench_caller!(c($));

    bench!(vec_deque_get, chunked_deque_get);
    bench!(vec_deque_read_front, chunked_deque_read_front);
    bench!(vec_deque_read_back, chunked_deque_read_back);
    bench!(vec_deque_iterate, chunked_deque_iterate);
    bench!(vec_deque_iterate_rev, chunked_deque_iterate_rev);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
