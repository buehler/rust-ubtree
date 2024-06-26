#![cfg(test)]

use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use rust_ubtree::UBTree;

fn get_tree_with_large_sets() -> UBTree<u32> {
    let mut rng = StdRng::seed_from_u64(1337);
    let mut sets = Vec::new();
    for _ in 0..1024 {
        let mut set = Vec::new();
        for _ in 0..rng.gen_range(0..1024) {
            let next = rng.gen_range(0..=32);
            if !set.contains(&next) {
                set.push(next);
            }
        }
        sets.push(set);
    }

    UBTree::new_with_data(&sets)
}

fn querying_exists(c: &mut Criterion) {
    let tree = get_tree_with_large_sets();
    let queries = vec![
        vec![],
        vec![15],
        vec![18, 19, 16, 15],
        vec![28, 17, 19, 30],
        vec![17, 256, 19, 128],
    ];

    let mut group = c.benchmark_group("querying exists unsorted");
    for q in &queries {
        group.bench_with_input(format!("query: {:?}", q), q, |b, q| {
            b.iter(|| tree.exists(q))
        });
    }
    group.finish();

    let mut group = c.benchmark_group("querying exists sorted");
    for mut q in queries {
        q.sort();
        group.bench_with_input(format!("query: {:?}", q), &q, |b, q| {
            b.iter(|| tree.exists(q))
        });
    }
    group.finish();
}

fn querying_subsets(c: &mut Criterion) {
    let tree = get_tree_with_large_sets();
    let queries = vec![
        vec![],
        vec![15],
        vec![22, 18],
        vec![31, 18, 24, 25, 16, 9, 11, 0, 8],
    ];

    let mut group = c.benchmark_group("querying subsets unsorted");
    for q in &queries {
        group.bench_with_input(format!("query: {:?}", q), q, |b, q| {
            b.iter(|| tree.subsets(q))
        });
    }
    group.finish();

    let mut group = c.benchmark_group("querying subsets sorted");
    for mut q in queries {
        q.sort();
        group.bench_with_input(format!("query: {:?}", q), &q, |b, q| {
            b.iter(|| tree.subsets(q))
        });
    }
    group.finish();
}

fn querying_supersets(c: &mut Criterion) {
    let tree = get_tree_with_large_sets();
    let queries = vec![vec![], vec![8], vec![10, 11, 0, 9], vec![3, 4, 8, 11]];

    let mut group = c.benchmark_group("querying supersets unsorted");
    for q in &queries {
        group.bench_with_input(format!("query: {:?}", q), q, |b, q| {
            b.iter(|| tree.supersets(q))
        });
    }
    group.finish();

    let mut group = c.benchmark_group("querying supersets sorted");
    for mut q in queries {
        q.sort();
        group.bench_with_input(format!("query: {:?}", q), &q, |b, q| {
            b.iter(|| tree.supersets(q))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    querying_exists,
    querying_subsets,
    querying_supersets,
);
criterion_main!(benches);
