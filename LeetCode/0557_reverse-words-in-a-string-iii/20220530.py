class Solution:
    def reverseWords(self, s: str) -> str:
        return " ".join(["".join(reversed(ss)) for ss in s.split()])