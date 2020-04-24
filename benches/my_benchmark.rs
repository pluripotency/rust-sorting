#[macro_use]
extern crate criterion;
extern crate rand;
extern crate rust_sorting;
use rand::Rng;
use criterion::Criterion;
use rust_sorting::bubble;
use rust_sorting::{merge_no_recurse, merge_recurse, merge_mine, quick, insertion};

fn criterion_benchmark(c: &mut Criterion){
    let test_data: Vec<usize> = (0..10000).map(|_| {
        rand::thread_rng().gen_range(1, std::usize::MAX)
    }).collect();

    let mut v = test_data.clone();
    c.bench_function("rust native merge sort", |b| b.iter(|| v.sort()));

    let mut v = test_data.clone();
    c.bench_function("rust native quick sort", |b| b.iter(|| v.sort_unstable()));

    let mut v = test_data.clone();
    c.bench_function("merge no recurse", |b| b.iter(|| merge_no_recurse::merge_sort(&mut v)));

    let mut v = test_data.clone();
    c.bench_function("merge recurse", |b| b.iter(|| merge_recurse::merge_sort(&mut v)));

    let mut v = test_data.clone();
    let length = v.len();
    c.bench_function("merge mine", |b| b.iter(|| merge_mine::merge_sort(&mut v, 0, length - 1)));

    let mut v = test_data.clone();
    c.bench_function("quick", |b| b.iter(|| quick::quick_sort(&mut v)));

    let mut v = test_data.clone();
    c.bench_function("insertion", |b| b.iter(|| insertion::insertion_sort(&mut v, &|a, b| a.lt(b))));

    let mut v = test_data.clone();
    c.bench_function("bubble", |b| b.iter(|| bubble::bubble_sort(&mut v, |a, b| a < b)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

