#[macro_use]
extern crate criterion;
extern crate rand;
extern crate rust_sorting;
use rand::Rng;
use criterion::Criterion;
use rust_sorting::{
    merge_native,
    insertion_native,
    insertion,
    merge_insertion,
    merge_no_recurse_rosetta,
    merge_recurse_rosetta,
    quick_rosetta,
    bubble
};

fn criterion_benchmark(c: &mut Criterion){
    let test_data: Vec<usize> = (0..10000).map(|_| {
        rand::thread_rng().gen_range(1, std::usize::MAX)
    }).collect();

    let mut v = test_data.clone();
    c.bench_function("merge recurse", |b| b.iter(|| merge_recurse_rosetta::merge_sort(&mut v)));

    let mut v = test_data.clone();
    c.bench_function("merge no recurse", |b| b.iter(|| merge_no_recurse_rosetta::merge_sort(&mut v)));

    let mut v = test_data.clone();
    c.bench_function("merge insertion", |b| b.iter(|| merge_insertion::merge_sort(&mut v)));

    let mut v = test_data.clone();
    c.bench_function("rust native merge sort", |b| b.iter(|| v.sort()));

    let mut v = test_data.clone();
    c.bench_function("merge native", |b| b.iter(|| merge_native::merge_sort(&mut v, |a, b| a.lt(b))));

    let mut v = test_data.clone();
    c.bench_function("insertion native", |b| b.iter(|| insertion_native::insertion_sort(&mut v, |a, b| a.lt(b))));

    let mut v = test_data.clone();
    c.bench_function("insertion", |b| b.iter(|| insertion::insertion_sort(&mut v, |a, b| a.lt(b))));

    let mut v = test_data.clone();
    c.bench_function("rust native quick sort", |b| b.iter(|| v.sort_unstable()));

    let mut v = test_data.clone();
    c.bench_function("quick", |b| b.iter(|| quick_rosetta::quick_sort(&mut v)));

    // let mut v = test_data.clone();
    // c.bench_function("quick_mine", |b| b.iter(|| quick_mine::quick_sort(&mut v, 0, v.len(), 1)));

    // let mut v = test_data.clone();
    // c.bench_function("bubble", |b| b.iter(|| bubble::bubble_sort(&mut v, |a, b| a < b)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

