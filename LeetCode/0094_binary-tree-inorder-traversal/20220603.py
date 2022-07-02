# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def inorderTraversal(self, root: Optional[TreeNode]) -> list[int]:
        self.answer = []
        self.traverse(root)
        return self.answer
        
    def traverse(self, node: Optional[TreeNode]):
        if node is None:
            return

        self.traverse(node.left)
        self.answer.append(node.val)
        self.traverse(node.right)