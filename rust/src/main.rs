use std::time::Instant;

extern crate nth_prime;

macro_rules! elapsed {
    ($p:path, $n:ident) => {{
        let now = Instant::now();
        let result = $p($n);
        let elapsed = now.elapsed();
        println!("{:>30}: {} ({:?})", std::stringify!($p), result, elapsed);
    }};
}
fn main() {
    if let Some(n) = std::env::args()
        .filter_map(|s| s.parse::<usize>().ok())
        .next()
    {
        elapsed!(nth_prime::trial_division, n);
        elapsed!(nth_prime::gmp, n);
        elapsed!(nth_prime::eratosthenes, n);
        elapsed!(nth_prime::eratosthenes_pi, n);
        elapsed!(nth_prime::eratosthenes_wf, n);
        elapsed!(nth_prime::atkin, n);
        elapsed!(nth_prime::primal, n);
    }
}
