# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


dp = {}


def f(l: int, r: int) -> list[TreeNode | None]:
    if l >= r:
        return [None]

    if (l, r) in dp:
        return dp[l, r]

    ans = []
    for i in range(l, r):
        left_nodes = f(l, i)
        right_nodes = f(i + 1, r)
        for left_node in left_nodes:
            for right_node in right_nodes:
                ans.append(TreeNode(i, left_node, right_node))

    dp[l, r] = ans
    return ans


class Solution:
    def generateTrees(self, n: int) -> List[Optional[TreeNode]]:
        return f(1, n + 1)
