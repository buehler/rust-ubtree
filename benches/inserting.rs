#![cfg(test)]

use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use rust_ubtree::UBTree;

fn insert_single_small_set(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(1337);
    let mut set = [0i32; 8];
    rng.fill(&mut set);

    c.bench_function("Insert single small set", |b| {
        b.iter_batched(
            || (UBTree::new(), set.clone()),
            |(mut tree, data)| tree.insert(&data),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn insert_multiple_small_sets(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(1337);
    let mut sets = [[0i32; 8]; 8];
    for set in &mut sets {
        rng.fill(set);
    }

    c.bench_function("Insert multiple small sets", |b| {
        b.iter_batched(
            || (UBTree::new(), sets.clone()),
            |(mut tree, data)| {
                for set in data {
                    tree.insert(&set);
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn insert_multiple_large_sets(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(1337);
    let mut sets = Vec::new();
    for _ in 0..1024 {
        let mut set = Vec::new();
        for _ in 0..1024 {
            set.push(rng.gen::<i32>());
        }
        sets.push(set);
    }

    c.bench_function("Insert multiple large sets", |b| {
        b.iter_batched(
            || (UBTree::new(), sets.clone()),
            |(mut tree, data)| {
                for set in data {
                    tree.insert(&set);
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    insert_single_small_set,
    insert_multiple_small_sets,
    insert_multiple_large_sets
);
criterion_main!(benches);
