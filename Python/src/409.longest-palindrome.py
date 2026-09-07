#
# @lc app=leetcode id=409 lang=python3
#
# [409] Longest Palindrome
#


# @lc code=start
class Solution:
    def longestPalindrome(self, s: str) -> int:
        from collections import Counter

        c = Counter(s)

        result = 0
        should_add_one = False
        for _, v in c.items():
            if v >= 2:
                result += (v // 2) * 2

            if v > 2 and v % 2 != 0:
                should_add_one = True
            elif v % 2 != 0:
                should_add_one = True

        if should_add_one:
            return result + 1

        return result


# @lc code=end
