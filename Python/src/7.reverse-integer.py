#
# @lc app=leetcode id=7 lang=python3
#
# [7] Reverse Integer
#


# @lc code=start
class Solution:
    def reverse(self, x: int) -> int:
        is_signed = x < 0
        x = x * -1 if is_signed else x
        x = list(str(x))
        x.reverse()
        x = int("".join(x))
        ans = x * -1 if is_signed else x
        
        if -2**31 <= ans <= 2**31 - 1:
            return ans
        else:
            return 0 


# @lc code=end
