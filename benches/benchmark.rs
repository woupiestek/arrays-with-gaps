use arrays_with_gaps::RedBlackTree;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn make_shuffle(n: usize) -> Vec<i32> {
    let mut keys: Vec<i32> = (0..n as i32).collect();
    let mut seed = 0x1234_5678u32;
    for i in (1..n).rev() {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        let j = (seed as usize) % (i + 1);
        keys.swap(i, j);
    }
    keys
}

fn bench_insert_ordered(c: &mut Criterion) {
    let keys: Vec<i32> = (0..2_000).collect();
    c.bench_function("rb_tree insert ordered", |b| {
        b.iter(|| {
            let mut tree = RedBlackTree::new();
            for &key in &keys {
                tree.insert(black_box(key), black_box(key));
            }
            black_box(tree);
        })
    });
}

fn bench_insert_random(c: &mut Criterion) {
    let keys = make_shuffle(2_000);
    c.bench_function("rb_tree insert random", |b| {
        b.iter(|| {
            let mut tree = RedBlackTree::new();
            for &key in &keys {
                tree.insert(black_box(key), black_box(key));
            }
            black_box(tree);
        })
    });
}

fn bench_get_random(c: &mut Criterion) {
    let keys = make_shuffle(2_000);
    let mut tree = RedBlackTree::new();
    for &key in &keys {
        tree.insert(key, key);
    }

    c.bench_function("rb_tree get random", |b| {
        b.iter(|| {
            for &key in &keys {
                black_box(tree.get(black_box(&key)));
            }
        })
    });
}

fn bench_remove_random(c: &mut Criterion) {
    let keys = make_shuffle(2_000);
    c.bench_function("rb_tree remove random", |b| {
        b.iter(|| {
            let mut tree = RedBlackTree::new();
            for &key in &keys {
                tree.insert(key, key);
            }
            for &key in &keys {
                black_box(tree.remove(black_box(&key)));
            }
        })
    });
}

fn criterion_benchmarks(c: &mut Criterion) {
    bench_insert_ordered(c);
    bench_insert_random(c);
    bench_get_random(c);
    bench_remove_random(c);
}

criterion_group!(benches, criterion_benchmarks);
criterion_main!(benches);
