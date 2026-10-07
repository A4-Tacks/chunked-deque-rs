use benches::*;

#[derive(Debug, Clone)]
struct Medium {
    _x: String,
    _f: [String; 6],
}

#[derive(Debug, Clone)]
struct Large {
    _x: String,
    _f: [String; 48],
}

fn random_vec_deque_as<T>(rng: &mut ThreadRng, f: fn(u32) -> T) -> VecDeque<T> {
    let mut deque = VecDeque::new();

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

fn random_chunked_deque_as<T>(rng: &mut ThreadRng, f: fn(u32) -> T) -> Deque<T> {
    let mut deque = Deque::new();

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

fn vec_deque_push_back<T: Clone>(b: &mut Bencher, f: fn(u32) -> T) {
    let mut rng = rand::rng();
    let mut datas = vec![];
    let deque = VecDeque::new();

    for _ in 0..VEC_SIZE {
        datas.push(f(rng.random::<u32>()));
    }
    b.iter_batched_ref(
        || deque.clone(),
        |deque| {
            for elem in datas.iter().cloned() {
                deque.push_back(elem);
            }
        },
        BATCH_SIZE,
    );
}

fn chunked_deque_push_back<T: Clone>(b: &mut Bencher, f: fn(u32) -> T) {
    let mut rng = rand::rng();
    let mut datas = vec![];
    let deque = Deque::new();

    for _ in 0..VEC_SIZE {
        datas.push(f(rng.random::<u32>()));
    }
    b.iter_batched_ref(
        || deque.clone(),
        |deque| {
            for elem in datas.iter().cloned() {
                deque.push_back(elem);
            }
        },
        BATCH_SIZE,
    );
}

fn vec_deque_pop_back<T: Clone>(b: &mut Bencher, f: fn(u32) -> T) {
    let mut rng = rand::rng();

    b.iter_batched_ref(
        || random_vec_deque_as(&mut rng, f),
        |deque| {
            while !deque.is_empty() {
                black_box(deque.pop_back());
            }
        },
        BATCH_SIZE,
    );
}

fn chunked_deque_pop_back<T: Clone>(b: &mut Bencher, f: fn(u32) -> T) {
    let mut rng = rand::rng();

    b.iter_batched_ref(
        || random_chunked_deque_as(&mut rng, f),
        |deque| {
            while !deque.is_empty() {
                black_box(deque.pop_back());
            }
        },
        BATCH_SIZE,
    );
}

fn vec_deque_push_back_string(b: &mut Bencher) {
    vec_deque_push_back(b, |it| it.to_string());
}
fn vec_deque_push_back_medium(b: &mut Bencher) {
    vec_deque_push_back(b, |it| Medium {
        _x: it.to_string(),
        _f: [const { String::new() }; 6],
    });
}
fn vec_deque_push_back_large(b: &mut Bencher) {
    vec_deque_push_back(b, |it| Large {
        _x: it.to_string(),
        _f: [const { String::new() }; 48],
    });
}
fn vec_deque_pop_back_string(b: &mut Bencher) {
    vec_deque_pop_back(b, |it| it.to_string());
}
fn vec_deque_pop_back_medium(b: &mut Bencher) {
    vec_deque_pop_back(b, |it| Medium {
        _x: it.to_string(),
        _f: [const { String::new() }; 6],
    });
}
fn vec_deque_pop_back_large(b: &mut Bencher) {
    vec_deque_pop_back(b, |it| Large {
        _x: it.to_string(),
        _f: [const { String::new() }; 48],
    });
}

fn chunked_deque_push_back_string(b: &mut Bencher) {
    chunked_deque_push_back(b, |it| it.to_string());
}
fn chunked_deque_push_back_medium(b: &mut Bencher) {
    chunked_deque_push_back(b, |it| Medium {
        _x: it.to_string(),
        _f: [const { String::new() }; 6],
    });
}
fn chunked_deque_push_back_large(b: &mut Bencher) {
    chunked_deque_push_back(b, |it| Large {
        _x: it.to_string(),
        _f: [const { String::new() }; 48],
    });
}
fn chunked_deque_pop_back_string(b: &mut Bencher) {
    chunked_deque_pop_back(b, |it| it.to_string());
}
fn chunked_deque_pop_back_medium(b: &mut Bencher) {
    chunked_deque_pop_back(b, |it| Medium {
        _x: it.to_string(),
        _f: [const { String::new() }; 6],
    });
}
fn chunked_deque_pop_back_large(b: &mut Bencher) {
    chunked_deque_pop_back(b, |it| Large {
        _x: it.to_string(),
        _f: [const { String::new() }; 48],
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_caller!(c($));

    bench!(vec_deque_push_back_string, chunked_deque_push_back_string);
    bench!(vec_deque_push_back_medium, chunked_deque_push_back_medium);
    bench!(vec_deque_push_back_large, chunked_deque_push_back_large);
    bench!(vec_deque_pop_back_string, chunked_deque_pop_back_string);
    bench!(vec_deque_pop_back_medium, chunked_deque_pop_back_medium);
    bench!(vec_deque_pop_back_large, chunked_deque_pop_back_large);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
