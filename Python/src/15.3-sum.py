#
# @lc app=leetcode id=15 lang=python3
#
# [15] 3Sum
#


# @lc code=start
class Solution:
    def threeSum(self, nums: list[int]) -> list[list[int]]:
        res = []
        nums.sort()

        for i, e in enumerate(nums):

            if i > 0 and e == nums[i - 1]:
                continue

            l, r = i + 1, len(nums) - 1

            while l < r:
                sum = nums[l] + nums[r] + e

                if sum == 0:
                    res.append([e, nums[l], nums[r]])
                    l += 1
                    while nums[l] == nums[l - 1] and l < r:
                        l += 1
                elif sum < 0:
                    l += 1
                elif sum > 0:
                    r -= 1

        return res


# @lc code=end
