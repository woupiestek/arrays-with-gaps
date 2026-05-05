use arrays_with_gaps::{Map, RedBlackTree, soarb_tree::SOARBTree};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

const TEST_SIZE: usize = 500;

// todo: do not use uniform distributions here!
fn make_shuffle(n: usize) -> Vec<i32> {
    let mut keys: Vec<i32> = (0..n)
        .flat_map(|i| vec![i as i32; n - i as usize])
        .collect();
    let mut seed = 0x1234_5678u32;
    for i in (1..keys.len()).rev() {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        let j = (seed as usize) % (i + 1);
        keys.swap(i, j);
    }
    keys
}

struct TestCase {
    name: String,
    factory: fn() -> Box<dyn Map<i32, i32>>,
}

// same tests, different types, how?

impl TestCase {
    fn new(name: String, factory: fn() -> Box<dyn Map<i32, i32>>) -> Self {
        Self { name, factory }
    }

    fn bench_insert_ordered(&self, c: &mut Criterion) {
        let keys: Vec<i32> = (0..TEST_SIZE as i32)
            .flat_map(|i| vec![i as i32; TEST_SIZE - i as usize])
            .collect();
        c.bench_function(&format!("{} insert ordered", self.name), |b| {
            b.iter(|| {
                let mut tree = (self.factory)();
                for &key in &keys {
                    tree.insert(black_box(key), black_box(key));
                }
                black_box(tree);
            })
        });
    }

    fn bench_insert_random(&self, c: &mut Criterion) {
        let keys = make_shuffle(TEST_SIZE);
        c.bench_function(&format!("{} insert random", self.name), |b| {
            b.iter(|| {
                let mut tree = (self.factory)();
                for &key in &keys {
                    tree.insert(black_box(key), black_box(key));
                }
                black_box(tree);
            })
        });
    }

    fn bench_get_random(&self, c: &mut Criterion) {
        let keys = make_shuffle(TEST_SIZE);
        let mut tree = (self.factory)();
        for &key in &keys {
            tree.insert(key, key);
        }

        c.bench_function(&format!("{} get random", self.name), |b| {
            b.iter(|| {
                for &key in &keys {
                    black_box(tree.get(black_box(&key)));
                }
            })
        });
    }

    fn bench_remove_random(&self, c: &mut Criterion) {
        let keys = make_shuffle(TEST_SIZE);
        c.bench_function(&format!("{} remove random", self.name), |b| {
            b.iter(|| {
                let mut tree = (self.factory)();
                for &key in &keys {
                    tree.insert(key, key);
                }
                for &key in &keys {
                    black_box(tree.remove(black_box(&key)));
                }
            })
        });
    }
}

fn criterion_benchmarks(c: &mut Criterion) {
    let rb_tree = TestCase::new("rb_tree".to_string(), || Box::new(RedBlackTree::new()));
    let soarb_tree = TestCase::new("soarb_tree".to_string(), || Box::new(SOARBTree::new()));
    rb_tree.bench_insert_ordered(c);
    soarb_tree.bench_insert_ordered(c);
    rb_tree.bench_insert_random(c);
    soarb_tree.bench_insert_random(c);
    rb_tree.bench_get_random(c);
    soarb_tree.bench_get_random(c);
    rb_tree.bench_remove_random(c);
    soarb_tree.bench_remove_random(c);
}

criterion_group!(benches, criterion_benchmarks);
criterion_main!(benches);
