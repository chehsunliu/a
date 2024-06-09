import sys


def f(nums: list[tuple[int, int]], x: int) -> list[int] | None:
    nums.sort()

    l, r = 0, len(nums) - 1

    while l < r:
        v = nums[l][0] + nums[r][0]
        if v == x:
            return [nums[r][1], nums[l][1]]
        elif v > x:
            r -= 1
        else:
            l += 1

    return None


def main():
    _, x_s = sys.stdin.readline().strip().split(" ")
    x = int(x_s)
    nums = [int(a) for a in sys.stdin.readline().strip().split(" ")]
    v = f([(num, i) for i, num in enumerate(nums)], x)
    if v is None:
        print("IMPOSSIBLE")
    else:
        print(v[0] + 1, v[1] + 1)


if __name__ == "__main__":
    main()
