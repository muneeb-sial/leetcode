#
# @lc app=leetcode id=287 lang=python3
#
# [287] Find the Duplicate Number
#


# @lc code=start
class Solution:
    def findDuplicate(self, nums: list[int]) -> int:
        s, f = 0, 0

        while True:
            s = nums[s]
            f = nums[nums[f]]
            if s == f:
                break

        intercept = 0

        while True:
            s = nums[s]
            intercept = nums[intercept]
            if intercept == s:
                return intercept


# @lc code=end
