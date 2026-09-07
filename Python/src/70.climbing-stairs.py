#
# @lc app=leetcode id=70 lang=python
#
# [70] Climbing Stairs
#


# @lc code=start
class Solution(object):
    def climbStairs(self, n):
        """
        :type n: int
        :rtype: int
        """

        if n <= 2:
            return n

        a, b = 1, 2
        for _ in range(3, n + 1):
            a, b = b, a + b

        return b


# @lc code=end
