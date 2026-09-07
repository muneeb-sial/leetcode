#
# @lc app=leetcode id=389 lang=python3
#
# [389] Find the Difference
#


# @lc code=start
class Solution:
    def findTheDifference(self, s: str, t: str) -> str:
        from collections import Counter

        cs = Counter(s)
        ct = Counter(t)
        i = ct - cs
        if len(i) != 0:
            return next(iter(i))
        else:
            return ""


# @lc code=end
