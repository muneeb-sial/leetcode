#
# @lc app=leetcode id=50 lang=python3
#
# [50] Pow(x, n)
#


# @lc code=start
class Solution:
    def myPow(self, x: float, n: int) -> float:
        # if n == 0:
        #     return 1
        # ans = x
        # if n > 0:
        #     for _ in range(1, n):
        #         ans = ans * x
        # if n < 0:
        #     ans = 1/ans
        #     for _ in range(1, (n * -1)):
        #         ans = ans / x
        # return ans
        return x**n


# @lc code=end
