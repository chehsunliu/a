class Item:
    def __init__(self, s: str) -> None:
        counter = [0 for _ in range(26)]
        for c in s:
            counter[ord(c) - ord("a")] += 1
            
        self.s = s
        self.feature = tuple(counter)
        
    def __hash__(self) -> int:
        return self.feature.__hash__()
    
    def __eq__(self, other: "Item") -> bool:
        return self.feature == other.feature
        

class Solution:
    def groupAnagrams(self, strs: list[str]) -> list[list[str]]:
        records = {}
        items = [Item(s) for s in strs]
        
        for item in items:
            if item not in records:
                records[item] = []
            records[item].append(item)
            
        return [[v.s for v in vs] for vs in records.values()]
