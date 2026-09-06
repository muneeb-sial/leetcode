#
# @lc app=leetcode id=41 lang=python3
#
# [41] First Missing Positive
#


# @lc code=start
class Solution:
    def firstMissingPositive(self, nums) -> int:
        s = set(nums)
        mx = max(s)
        ans = 1

        while ans <= mx:
            if ans not in s:
                return ans
            ans += 1

        return ans


# @lc code=end
