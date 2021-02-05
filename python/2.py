import cProfile
import io
import pstats
import sys
from typing import List


def is_prime(num: int, primes: List[int]) -> bool:
    for p in primes:
        if num % p == 0:
            return False

    primes.append(num)
    return True


def main(n: int) -> int:
    i = 0
    primes: List[int] = []
    for p in range(2, 1000000):
        if is_prime(p, primes):
            i += 1
        if i == n:
            return p
    raise ValueError


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
