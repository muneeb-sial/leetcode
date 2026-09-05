#
# @lc app=leetcode id=367 lang=python3
#
# [367] Valid Perfect Square
#


# @lc code=start
class Solution:
    def isPerfectSquare(self, num: int) -> bool:
        l, r = 0, num
        while l <= r:
            mid = int((l + r) / 2)
            square = mid * mid
            
            if square == num:
                return True

            if square > num:
                r = mid - 1
            elif square < num:
                l = mid + 1

        return False


# @lc code=end
