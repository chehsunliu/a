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
        count = 0
        count_limit = 1
        prev_node = None

        while len(queue) != 0:
            node = queue.popleft()
            if node.left is not None:
                queue.append(node.left)
            if node.right is not None:
                queue.append(node.right)

            count += 1
            if prev_node is not None:
                prev_node.next = node
            prev_node = node

            if count == count_limit:
                count = 0
                count_limit *= 2
                prev_node = None

        return root
