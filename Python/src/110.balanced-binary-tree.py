#
# @lc app=leetcode id=110 lang=python3
#
# [110] Balanced Binary Tree
#


# @lc code=start
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        if not root:
            return True

        def dfs(root):
            if not root:
                return [True, 0]
            l, r = dfs(root.left), dfs(root.right)

            balanced = l[0] and r[0] and abs(l[1] - r[1]) <= 1
            return [balanced, max(l[1], r[1]) + 1]

        return dfs(root)[0]


# @lc code=end
