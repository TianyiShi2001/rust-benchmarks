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
fn bench_concat_append_then_forloop(b: &mut Bencher) {
    b.iter(|| {
        let mut rng = rand::thread_rng();
        let mut x: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        let mut y: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        x.append(&mut y);
        for e in x {
            e + 1;
        }
    });
}

#[bench]
fn bench_concat_extend_then_forloop(b: &mut Bencher) {
    b.iter(|| {
        let mut rng = rand::thread_rng();
        let mut x: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        let y: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        x.extend(y);
        for e in x {
            e + 1;
        }
    });
}

#[bench]
fn bench_concat_concat_then_forloop(b: &mut Bencher) {
    b.iter(|| {
        let mut rng = rand::thread_rng();
        let x: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        let y: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        for e in [x, y].concat() {
            e + 1;
        }
    });
}

#[bench]
fn bench_concat_iter_chain_then_forloop(b: &mut Bencher) {
    b.iter(|| {
        let mut rng = rand::thread_rng();
        let x: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        let y: Vec<i32> = (0..10000).map(|_| rng.gen_range(0, 20)).collect();
        for e in x.into_iter().chain(y.into_iter()) {
            e + 1;
        }
    });
}
