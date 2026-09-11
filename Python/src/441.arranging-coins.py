#
# @lc app=leetcode id=441 lang=python3
#
# [441] Arranging Coins
#


# @lc code=start
class Solution:
    def arrangeCoins(self, n: int) -> int:
        n = n
        ans = 0
        i = 1

        while n - i >= 0:
            n = n - i
            ans += 1
            i += 1

        return ans 


# @lc code=end
