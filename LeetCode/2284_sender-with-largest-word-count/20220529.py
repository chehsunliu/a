class Solution:
    def largestWordCount(self, messages: list[str], senders: list[str]) -> str:
        records = {}
        
        for message, sender in zip(messages, senders):
            if sender not in records:
                records[sender] = 0
                
            records[sender] += len(message.strip().split())
            
        max_count = 0
        names = []
        for name, count in records.items():
            if count > max_count:
                max_count = count
                names = [name]
            elif count == max_count:
                names.append(name)
                
        return sorted(names)[-1]