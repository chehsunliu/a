def search(arr: list[str], target: str) -> int:
    l, r = 0, len(arr) - 1
        
    while l <= r:
        m = (l + r) // 2

        if arr[m] == target:
            return m
        elif arr[m] < target:
            l = m + 1
        else:
            r = m - 1

    return l


class Solution:
    def nextGreatestLetter(self, letters: list[str], target: str) -> str:
        target_next = chr(ord(target) + 1)

        index = search(letters, target_next)
        if index < len(letters):
            return letters[index]
        
        return letters[0]