pub fn trial_division(n: usize) -> u32 {
    let mut primes = Vec::with_capacity(n);
    primes.push(2u32);
    while primes.len() <= n {
        if let Some(prime) = (primes[primes.len() - 1] + 1..).find(|&m| {
            primes
                .iter()
                .take_while(|&e| e * e <= m)
                .all(|&e| m % e != 0)
        }) {
            primes.push(prime);
        }
    }
    primes[n - 1]
}

pub fn gmp(n: usize) -> u32 {
    let mut mpz = gmp::mpz::Mpz::new();
    for _ in 0..n {
        mpz = mpz.nextprime();
    }
    mpz.to_string().parse().unwrap()
}

pub fn eratosthenes(n: usize) -> u32 {
    fn list_primes(limit: usize) -> Vec<u32> {
        let mut primes = Vec::new();
        let mut is_prime = vec![true; limit + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        for p in 0..=limit {
            if !is_prime[p] {
                continue;
            }
            primes.push(p as u32);
            for i in (p * p..=limit).step_by(p) {
                is_prime[i] = false;
            }
        }
        primes
    }

    let mut limit = 1000;
    loop {
        let primes = list_primes(limit);
        if primes.len() >= n {
            return primes[n - 1];
        }
        limit *= 2;
    }
}

pub fn eratosthenes_pi(n: usize) -> u32 {
    let n_ = n as f64;
    let lg = n_.ln();
    let limit = std::cmp::max(100, (n_ * lg * 1.2) as usize);

    let mut primes = Vec::new();
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 0..=limit {
        if !is_prime[p] {
            continue;
        }
        primes.push(p as u32);
        if primes.len() == n {
            return primes[n - 1];
        }
        for i in (p * p..=limit).step_by(p) {
            is_prime[i] = false;
        }
    }
    unreachable!();
}

pub fn eratosthenes_wf(n: usize) -> u32 {
    let n_ = n as f64;
    let lg = n_.ln();

    let limit = std::cmp::max(100, (n_ * lg * 1.2) as usize);

    let mut primes = vec![2, 3, 5];
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let inc = [6, 4, 2, 4, 2, 4, 6, 2];
    let mut p = 1;
    for i in 0.. {
        p += inc[i & 7];
        if p >= limit {
            break;
        }
        if !is_prime[p] {
            continue;
        }
        primes.push(p as u32);
        if primes.len() >= n {
            return primes[n - 1];
        }
        for j in (p * p..=limit).step_by(p) {
            is_prime[j] = false;
        }
    }
    unreachable!();
}

pub fn atkin(n: usize) -> u32 {
    fn list_primes(limit: usize) -> Vec<u32> {
        let mut primes = Vec::new();
        if limit > 2 {
            primes.push(2);
        }
        if limit > 3 {
            primes.push(3);
        }
        let mut sieve = vec![false; limit];
        for x in (1..).take_while(|&x| x * x < limit) {
            for y in (1..).take_while(|&y| y * y < limit) {
                {
                    let n = (4 * x * x) + (y * y);
                    if n <= limit && (n % 12 == 1 || n % 12 == 5) {
                        sieve[n] ^= true;
                    }
                }
                {
                    let n = (3 * x * x) + (y * y);
                    if n <= limit && n % 12 == 7 {
                        sieve[n] ^= true;
                    }
                }
                if x > y {
                    let n = (3 * x * x) - (y * y);
                    if n <= limit && n % 12 == 11 {
                        sieve[n] ^= true;
                    }
                }
            }
        }
        for r in (5..).take_while(|&r| r * r < limit) {
            if sieve[r] {
                for i in (1..).map(|i| i * r * r).take_while(|&i| i < limit) {
                    sieve[i] = false;
                }
            }
        }
        primes.extend(
            sieve
                .iter()
                .enumerate()
                .filter_map(|(i, &b)| if b { Some(i as u32) } else { None }),
        );
        primes
    }

    let mut limit = 1000;
    loop {
        let primes = list_primes(limit);
        if primes.len() >= n {
            return primes[n - 1];
        }
        limit *= 2;
    }
}

pub fn primal(n: usize) -> u32 {
    primal::StreamingSieve::nth_prime(n) as u32
}
