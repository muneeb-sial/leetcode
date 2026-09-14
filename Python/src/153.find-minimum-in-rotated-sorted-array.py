#
# @lc app=leetcode id=153 lang=python3
#
# [153] Find Minimum in Rotated Sorted Array
#


# @lc code=start
class Solution:
    def findMin(self, nums: List[int]) -> int:
        ans = nums[0]
        l, r = 0, len(nums) - 1

        while l <= r:
            if nums[l] <= nums[r]:
                ans = min(ans, nums[l])
                break

            mid = (l + r) // 2
            ans = min(nums[mid], ans)
            if nums[mid] >= nums[l]:
                l = mid + 1
            else:
                r = mid - 1

        return ans


# @lc code=end
