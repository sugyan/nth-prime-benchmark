# nth-prime-benchmark

## Rust

```shell
cd ./rust
rustup run nightly cargo bench
```

```
test bench_100000_atkin           ... bench:  12,985,878 ns/iter (+/- 1,071,565)
test bench_100000_eratosthenes    ... bench:  27,902,525 ns/iter (+/- 1,104,242)
test bench_100000_eratosthenes_pi ... bench:   9,845,529 ns/iter (+/- 508,106)
test bench_100000_eratosthenes_wf ... bench:   5,217,517 ns/iter (+/- 374,603)
test bench_100000_primal          ... bench:     127,648 ns/iter (+/- 14,240)
test bench_10000_atkin            ... bench:     873,542 ns/iter (+/- 83,950)
test bench_10000_eratosthenes     ... bench:   1,653,995 ns/iter (+/- 166,616)
test bench_10000_eratosthenes_pi  ... bench:     764,175 ns/iter (+/- 79,996)
test bench_10000_eratosthenes_wf  ... bench:     387,469 ns/iter (+/- 40,348)
test bench_10000_gmp              ... bench:  16,731,627 ns/iter (+/- 1,398,410)
test bench_10000_primal           ... bench:       8,594 ns/iter (+/- 826)
test bench_10000_trial_division   ... bench:   2,107,299 ns/iter (+/- 198,683)
```