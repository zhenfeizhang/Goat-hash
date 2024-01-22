use std::time::Instant;

use ark_std::test_rng;
use ff::Field;
use goldilocks::Goldilocks;
use poseidon::Poseidon;

use crate::{ring::RingElement, GoatHash};

#[test]
fn bench_goat_hash() {
    let mut rng = test_rng();
    let repeat = 100;
    let repeat_per_hash = 1000;
    let mut hashers = vec![];
    let mut messages_vec = vec![];
    let mut res = vec![];

    for _ in 0..repeat {
        hashers.push(GoatHash::init(&mut rng));
        let mut messages = vec![];
        for _ in 0..repeat_per_hash {
            messages.push([
                RingElement::random_message(&mut rng),
                RingElement::random_message(&mut rng),
                RingElement::random_message(&mut rng),
                RingElement::random_message(&mut rng),
            ])
        }
        messages_vec.push(messages)
    }
    let start = Instant::now();
    for (hasher, messages) in hashers.iter().zip(messages_vec.iter()) {
        for msg in messages.iter() {
            res.push(hasher.hash(msg));
        }
    }
    println!(
        "goat hash cost {:?} ",
        start.elapsed() / repeat / repeat_per_hash
    );
}

#[test]
fn bench_poseidon_hash() {
    let hasher = Poseidon::<Goldilocks, 12, 11>::new(8, 22);

    let mut rng = test_rng();
    let mut messages = vec![];
    let mut res = vec![];

    let repeat_per_hash = 1000;
    for _ in 0..repeat_per_hash {
        messages.push([Goldilocks::random(&mut rng), Goldilocks::random(&mut rng)])
    }

    let start = Instant::now();
    for messages in messages.iter() {
        let mut tmp = hasher.clone();
        tmp.update(messages);
        res.push(tmp.squeeze());
    }
    println!(
        "poseidon hash cost {:?} ",
        start.elapsed() / repeat_per_hash
    );
}
