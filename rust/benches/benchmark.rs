#![feature(test)]
extern crate nth_prime;
extern crate test;

use test::Bencher;

#[bench]
fn bench_10000_trial_division(b: &mut Bencher) {
    b.iter(|| nth_prime::trial_division(10_000));
}

#[bench]
fn bench_10000_gmp(b: &mut Bencher) {
    b.iter(|| nth_prime::gmp(10_000));
}

#[bench]
fn bench_10000_eratosthenes(b: &mut Bencher) {
    b.iter(|| nth_prime::eratosthenes(10_000));
}

#[bench]
fn bench_10000_eratosthenes_pi(b: &mut Bencher) {
    b.iter(|| nth_prime::eratosthenes_pi(10_000));
}

#[bench]
fn bench_10000_eratosthenes_wf(b: &mut Bencher) {
    b.iter(|| nth_prime::eratosthenes_wf(10_000));
}

#[bench]
fn bench_10000_atkin(b: &mut Bencher) {
    b.iter(|| nth_prime::atkin(10_000));
}

#[bench]
fn bench_10000_primal(b: &mut Bencher) {
    b.iter(|| nth_prime::primal(10_000));
}

#[bench]
fn bench_100000_eratosthenes(b: &mut Bencher) {
    b.iter(|| nth_prime::eratosthenes(100_000));
}

#[bench]
fn bench_100000_eratosthenes_pi(b: &mut Bencher) {
    b.iter(|| nth_prime::eratosthenes_pi(100_000));
}

#[bench]
fn bench_100000_eratosthenes_wf(b: &mut Bencher) {
    b.iter(|| nth_prime::eratosthenes_wf(100_000));
}

#[bench]
fn bench_100000_atkin(b: &mut Bencher) {
    b.iter(|| nth_prime::atkin(100_000));
}

#[bench]
fn bench_100000_primal(b: &mut Bencher) {
    b.iter(|| nth_prime::primal(100_000));
}
