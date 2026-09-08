#
# @lc app=leetcode id=485 lang=python3
#
# [485] Max Consecutive Ones
#

# @lc code=start
class Solution:
    def findMaxConsecutiveOnes(self, nums) -> int:
        for i in range(0,len(nums)):
            nums[i] = str(nums[i])

        l = "".join(nums)
        l = l.split("0") 

        count = 0
        for i in l:
            count = max(count,len(i))
        return count
        
# @lc code=end
