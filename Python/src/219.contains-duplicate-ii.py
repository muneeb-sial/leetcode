#
# @lc app=leetcode id=219 lang=python
#
# [219]
#


# @lc code=start
class Solution(object):
    def containsNearbyDuplicate(self, nums, k):
        """
        :type nums: List[int]
        :type k: int
        :rtype: bool
        """
        last_seen = {}

        for i in range(0, len(nums)):
            if nums[i] in last_seen:
                if abs(i - last_seen[nums[i]]) <= k:
                    return True
            last_seen[nums[i]] = i

        return False


# @lc code=end
