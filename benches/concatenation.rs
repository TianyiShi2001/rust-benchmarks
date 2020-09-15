#![feature(test)]

extern crate test;
use rand::prelude::*;

use test::Bencher;

#[bench]
fn bench_concat___init__(b: &mut Bencher) {
    b.iter(|| {
        let mut rng = rand::thread_rng();
        let x: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        let y: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
    });
}

#[bench]
fn bench_concat_append(b: &mut Bencher) {
    b.iter(|| {
        let mut rng = rand::thread_rng();
        let mut x: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        let mut y: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        x.append(&mut y)
    });
}

#[bench]
fn bench_concat_extend(b: &mut Bencher) {
    b.iter(|| {
        let mut rng = rand::thread_rng();
        let mut x: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        let y: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        x.extend(y)
    });
}

#[bench]
fn bench_concat_concat(b: &mut Bencher) {
    b.iter(|| {
        let mut rng = rand::thread_rng();
        let x: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        let y: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        [x, y].concat()
    });
}

#[bench]
fn bench_concat_iter_chain(b: &mut Bencher) {
    b.iter(|| {
        let mut rng = rand::thread_rng();
        let x: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        let y: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        x.into_iter().chain(y.into_iter())
    });
}

#[bench]
fn bench_concat_iter_chain_collect(b: &mut Bencher) {
    b.iter(|| {
        let mut rng = rand::thread_rng();
        let x: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        let y: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        x.into_iter().chain(y.into_iter()).collect::<Vec<i32>>()
    });
}
