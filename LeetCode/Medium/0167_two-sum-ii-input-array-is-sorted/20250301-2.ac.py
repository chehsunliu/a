import sys


def main():
    t = int(sys.stdin.readline().strip())

    for line in sys.stdin:
        n = int(line.strip())
        arr = [int(s) for s in sys.stdin.readline().strip().split(" ")]

        counts = {}
        for a in arr:
            counts[a] = counts.get(a, 0) + 1

        ans = (0, 0)
        for k, v in counts.items():
            if v > ans[1]:
                ans = (k, v)
            elif v == ans[1] and k < ans[0]:
                ans = (k, v)

        print(ans[0], ans[1])


if __name__ == "__main__":
    main()
