import cProfile
import io
import pstats
import sys


def main(n: int) -> int:
    i = 0
    for p in range(2, 1000000):
        for q in range(2, p):
            if p % q == 0:
                break
        else:
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
