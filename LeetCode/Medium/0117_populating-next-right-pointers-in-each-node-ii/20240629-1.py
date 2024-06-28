"""
# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
"""

from collections import deque


class Solution:
    def connect(self, root: 'Optional[Node]') -> 'Optional[Node]':
        if root is None:
            return None

        queue = deque([root])
        values = {id(root): 1}

        prev_node = None
        prev_value = 0

        while len(queue) != 0:
            node = queue.popleft()
            value = values[id(node)]

            if node.left is not None:
                queue.append(node.left)
                values[id(node.left)] = value + 1
            if node.right is not None:
                queue.append(node.right)
                values[id(node.right)] = value + 1

            if prev_node is not None and prev_value == value:
                prev_node.next = node

            prev_node = node
            prev_value = value

        return root
