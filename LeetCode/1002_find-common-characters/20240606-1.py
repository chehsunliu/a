class Solution:
    def commonChars(self, words: List[str]) -> List[str]:
        summaries = []

        for word in words:
            summary = {}

            for c in word:
                summary[c] = summary.get(c, 0) + 1

            summaries.append(summary)

        ans = {}

        summary0 = summaries[0]

        for c, count in summary0.items():
            min_count = count
            for summary in summaries:
                if c in summary:
                    min_count = min(min_count, summary[c])
                else:
                    min_count = 0
                    break
            if min_count > 0:
                ans[c] = min_count

        output = []
        for c, count in ans.items():
            output += [c for _ in range(count)]

        return output
