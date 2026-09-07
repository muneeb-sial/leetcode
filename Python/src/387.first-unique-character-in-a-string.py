# @lc app=leetcode id=387 lang=python3
#
# [387] First Unique Character in a String
#


# @lc code=start
class Solution:
    def firstUniqChar(self, s: str) -> int:
        from collections import Counter
        c = Counter(list(s))
        
        for k, v in c.items():
            if v == 1:
                return s.index(k)
        
        return -1


# @lc code=end
