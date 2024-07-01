"""
# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
"""

from typing import Optional


class Solution:
    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:
        visited = {}

        def f(node: Optional['Node']) -> Optional['Node']:
            if node is None:
                return None
            if id(node) in visited:
                return visited[id(node)]

            new_node = Node(val=node.val)
            visited[id(node)] = new_node

            for neighbor in node.neighbors:
                new_neighbor = f(neighbor)
                if new_neighbor:
                    new_node.neighbors.append(new_neighbor)

            return new_node

        return f(node)