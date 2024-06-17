from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def solve(node: TreeNode | None, target: float, ans: TreeNode) -> int:
    if node is None:
        return ans.val

    diff = abs(target - node.val)
    old_diff = abs(target - ans.val)
    ans = node if diff < old_diff or (diff == old_diff and node.val < ans.val) else ans

    if node.left and target < node.val:
        return solve(node.left, target, ans)
    elif node.right and target > node.val:
        return solve(node.right, target, ans)
    else:
        return ans.val


class Solution:
    def closestValue(self, root: Optional[TreeNode], target: float) -> int:
        return solve(root, target, root)
