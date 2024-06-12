class Solution:
    def isNStraightHand(self, hand: List[int], groupSize: int) -> bool:
        distinct_numbers = sorted(list(set(hand)))

        counts = {}
        for v in hand:
            counts[v] = counts.get(v, 0) + 1

        for v in distinct_numbers:
            while counts[v] > 0:
                for i in range(groupSize):
                    if counts.get(v + i, 0) == 0:
                        return False

                    counts[v + i] -= 1

        return True
