import cProfile
import io
import pstats
import sys
from typing import List


def list_primes(limit: int) -> List[int]:
    primes = []
    is_prime = [True] * (limit + 1)
    is_prime[0] = False
    is_prime[1] = False

    for p in range(0, limit + 1):
        if not is_prime[p]:
            continue
        primes.append(p)
        for i in range(p * p, limit + 1, p):
            is_prime[i] = False

    return primes


def main(n: int) -> int:
    limit = 1000
    while True:
        primes = list_primes(limit)
        if len(primes) > n:
            return primes[n - 1]
        limit *= 2


if __name__ == "__main__":
    n = int(sys.argv[1])
    with cProfile.Profile() as pr:
        for _ in range(10):
            result = main(n)
    print(result)

    s = io.StringIO()
    ps = pstats.Stats(pr, stream=s)
    ps.print_stats("main")
    print(s.getvalue())
