use benches::*;

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

fn criterion_benchmark(c: &mut Criterion) {
    bench_caller!(c($));

    bench!(vec_deque_push_front, chunked_deque_push_front);
    bench!(vec_deque_push_back, chunked_deque_push_back);
    bench!(vec_deque_pop_front, chunked_deque_pop_front);
    bench!(vec_deque_pop_back, chunked_deque_pop_back);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
