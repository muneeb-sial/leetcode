#
# @lc app=leetcode id=383 lang=python3
#
# [383] Ransom Note
#


# @lc code=start
class Solution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        from collections import Counter

        r = Counter(ransomNote)
        m = Counter(magazine)

        for k, v in r.items():
            if m[k] < v:
                return False
            
        return True


# @lc code=end
