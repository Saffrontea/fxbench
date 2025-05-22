use std::collections::HashMap;
use rustc_hash::FxHashMap;
use rand::{Rng};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const NUM_ITEMS: usize = 100000;

fn generate_string_keys() -> Vec<String> {
    let mut rng = rand::thread_rng();
    (0..NUM_ITEMS)
        .map(|_| {
            let key: String = (0..16)
                .map(|_| rng.gen_range(b'a'..=b'z') as char)
                .collect();
            key
        })
        .collect()
}

fn bench_std_hashmap_string_keys(c: &mut Criterion) {
    let keys = generate_string_keys();
    c.bench_function("std HashMap insert (String keys)", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for k in &keys {
                map.insert(black_box(k.clone()), 1u32);
            }
        })
    });
}

fn bench_fx_hashmap_string_keys(c: &mut Criterion) {
    let keys = generate_string_keys();
    c.bench_function("FxHashMap insert (String keys)", |b| {
        b.iter(|| {
            let mut map = FxHashMap::default();
            for k in &keys {
                map.insert(black_box(k.clone()), 1u32);
            }
        })
    });
}

criterion_group!(
    benches,
    bench_std_hashmap_string_keys,
    bench_fx_hashmap_string_keys,
);
criterion_main!(benches);

