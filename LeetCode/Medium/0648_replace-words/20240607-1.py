class Solution:
    def replaceWords(self, dictionary: List[str], sentence: str) -> str:
        words = []
        dictionary.sort(key=lambda w: (len(w), w[0]))

        dp = {}

        for derivative in sentence.split(" "):
            if derivative in dp:
                words.append(dp[derivative])
                continue

            word = None
            for root in dictionary:
                if derivative.startswith(root):
                    word = root
                    break
            if word is None:
                word = derivative

            dp[derivative] = word
            words.append(word)

        return " ".join(words)
