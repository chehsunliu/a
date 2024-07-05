import sys


def search_l(arr: list[tuple[int, int]], x: int) -> int:
    l, r = 0, len(arr) - 1

    while l <= r and 0 <= r and l < len(arr):
        m = (l + r) // 2

        if x <= arr[m][0]:
            r = m - 1
        else:
            l = m + 1

    return l


def f(i: int, movies: list[tuple[int, int]], dp: dict[int, int]) -> int:
    if i in dp:
        return dp[i]

    _, b = movies[i]
    next_i = search_l(movies[i + 1 :], b) + i + 1

    ans = 0
    for j in range(next_i, len(movies)):
        ans = max(ans, f(j, movies, dp))

    dp[i] = ans + 1
    return ans + 1


def main():
    sys.stdin.readline()

    movies = []
    for line in sys.stdin:
        a, b = line.rstrip().split(" ")
        movies.append((int(a), int(b)))

    movies.sort()
    dp = {}

    ans = 0
    for i in range(len(movies)):
        ans = max(ans, f(i, movies, dp))

    print(ans)


if __name__ == "__main__":
    main()
