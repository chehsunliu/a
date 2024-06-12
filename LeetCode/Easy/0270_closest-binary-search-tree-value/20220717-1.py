# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def closestValue(self, root: Optional[TreeNode], target: float) -> int:
        self.ans = root.val
        self.f(root, target)
        return self.ans
    
    def f(self, node: Optional[TreeNode], target: float):
        if node is None:
            return
        
        if abs(node.val - target) < abs(self.ans - target):
            self.ans = node.val
            
        if target > node.val:
            self.f(node.right, target)
        else:
            self.f(node.left, target)
        