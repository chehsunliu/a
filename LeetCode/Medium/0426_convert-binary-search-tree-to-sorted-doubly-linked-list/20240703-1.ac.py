"""
# Definition for a Node.
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
"""


def traverse(node: 'Optional[Node]') -> 'Optional[Node]':
    left_node = node.left
    right_node = node.right

    if left_node is not None and right_node is not None:
        l1, l2 = traverse(left_node)
        r1, r2 = traverse(right_node)

        node.left = l2
        l2.right = node
        node.right = r1
        r1.left = node

        return l1, r2

    elif left_node is not None:
        l1, l2 = traverse(left_node)
        node.left = l2
        l2.right = node
        return l1, node

    elif right_node is not None:
        r1, r2 = traverse(right_node)
        node.right = r1
        r1.left = node
        return node, r2

    else:
        return node, node


class Solution:
    def treeToDoublyList(self, root: 'Optional[Node]') -> 'Optional[Node]':
        if root is None:
            return None

        n1, n2 = traverse(root)
        n1.left = n2
        n2.right = n1
        return n1
