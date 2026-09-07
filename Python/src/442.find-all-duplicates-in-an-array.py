#
# @lc app=leetcode id=442 lang=python3
#
# [442] Find All Duplicates in an Array
#


# @lc code=start
class Solution:
    def findDuplicates(self, nums: List[int]) -> List[int]:
        from collections import Counter

        c = Counter(nums)
        return [k for k, v in c.items() if v > 1]


# @lc code=end
