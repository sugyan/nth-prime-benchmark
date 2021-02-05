import cProfile
import io
import math
import pstats
import sys
from typing import List


def faked_eratosthenes(limit: int) -> List[int]:
    nums = [i for i in range(2, limit + 1)]
    primes = []
    while True:
        p = min(nums)
        if p > math.sqrt(limit):
            break
        primes.append(p)
        i = 0
        while i < len(nums):
            if nums[i] % p == 0:
                nums.pop(i)
                continue
            i += 1
    return primes + nums


def main(n: int) -> int:
    limit = 1000
    while True:
        primes = faked_eratosthenes(limit)
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
