from bisect import bisect_left


class Item:
    def __init__(self, w: int, h: int):
        self.w = w
        self.h = h


class Solution:
    def maxEnvelopes(self, envelopes: list[list[int]]) -> int:
        items = [Item(w=e[0], h=e[1]) for e in envelopes]
        items = sorted(items, key=lambda x: (x.w, -x.h))
        
        answers = []
        for item in items:
            i = bisect_left(answers, item.h)
            if i < len(answers):
                answers[i] = item.h
            else:
                answers.append(item.h)
                
        return len(answers)

        
#    (6, 5)
#    (5, 6)
# (3, 2) (2, 3) (4, 7)