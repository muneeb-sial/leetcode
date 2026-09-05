#
# @lc app=leetcode id=374 lang=python3
#
# [374] Guess Number Higher or Lower
#


# @lc code=start
# The guess API is already defined for you.
# @param num, your guess
# @return -1 if num is higher than the picked number
#          1 if num is lower than the picked number
#          otherwise return 0
# def guess(num: int) -> int:

def guess(num: int) -> int:
    pick = 6
    if num > pick:
        return -1
    elif num < pick:
        return 1
    else:
        return 0


class Solution:
    def guessNumber(self, n: int) -> int:
        l, r = 0, n

        while l <= r:
            mid = (l + r) // 2
            guessed = guess(mid)  # -1,0,1

            if guessed == 0:
                return mid
            elif guessed == 1:
                l = mid + 1
            else:
                r = mid - 1


# @lc code=end
