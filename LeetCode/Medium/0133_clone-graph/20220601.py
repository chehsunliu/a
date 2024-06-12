"""
# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
"""

class Solution:
    def cloneGraph(self, node: 'Node') -> 'Node':
        if node is None:
            return None

        self.dp = [None for _ in range(101)]
        return self.f(node)
        
    def f(self, node: 'Node'):
        if self.dp[node.val] is not None:
            return self.dp[node.val]
        
        cloned_node = Node(node.val)
        self.dp[node.val] = cloned_node
        
        for neighbor in node.neighbors:
            cloned_node.neighbors.append(self.f(neighbor))
            
        return cloned_node