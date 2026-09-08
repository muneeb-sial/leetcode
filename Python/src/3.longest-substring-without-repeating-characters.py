#
# @lc app=leetcode id=3 lang=python3
#
# [3] Longest Substring Without Repeating Characters
#


# @lc code=start
class Solution:
    # abcabcbb
    def lengthOfLongestSubstring(self, s: str) -> int:
        result = 0
        remember = set()
        p1 = 0

        for p2 in range(0, len(s)):
            while s[p2] in remember:
                remember.remove(s[p1])
                p1 += 1
            remember.add(s[p2])
            result = max(result, p2 - p1 + 1)
        return result


# @lc code=end
