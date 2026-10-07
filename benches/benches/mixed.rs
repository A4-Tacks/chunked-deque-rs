use benches::*;

fn random_linked_list_deque(rng: &mut ThreadRng) -> LinkedList<u32> {
    let mut deque = LinkedList::new();

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

fn vec_deque_pop_mixed(b: &mut Bencher) {
    let mut rng = rand::rng();
    let mut datas = vec![];

    for _ in 0..VEC_SIZE {
        datas.push(rng.random());
    }
    b.iter_batched_ref(
        || random_vec_deque(&mut rng),
        |deque| {
            for &data in &datas {
                if data {
                    black_box(deque.pop_back());
                } else {
                    black_box(deque.pop_front());
                }
            }
        },
        BATCH_SIZE,
    );
}

fn chunked_deque_pop_mixed(b: &mut Bencher) {
    let mut rng = rand::rng();
    let mut datas = vec![];

    for _ in 0..VEC_SIZE {
        datas.push(rng.random());
    }
    b.iter_batched_ref(
        || random_vec_deque(&mut rng),
        |deque| {
            for &data in &datas {
                if data {
                    black_box(deque.pop_back());
                } else {
                    black_box(deque.pop_front());
                }
            }
        },
        BATCH_SIZE,
    );
}

fn linked_list_deque_push_back(b: &mut Bencher) {
    let mut rng = rand::rng();
    let mut datas = vec![];
    let deque = LinkedList::new();

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

fn linked_list_deque_pop_back(b: &mut Bencher) {
    let mut rng = rand::rng();

    b.iter_batched_ref(
        || random_linked_list_deque(&mut rng),
        |deque| {
            while !deque.is_empty() {
                black_box(deque.pop_back());
            }
        },
        BATCH_SIZE,
    );
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_caller!(c($));

    bench!(vec_deque_push_mixed, chunked_deque_push_mixed);
    bench!(vec_deque_pop_mixed, chunked_deque_pop_mixed);
    bench!(linked_list_deque_push_back);
    bench!(linked_list_deque_pop_back);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
