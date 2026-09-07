#
# @lc app=leetcode id=520 lang=python3
#
# [520] Detect Capital
#

# @lc code=start
class Solution:
    def detectCapitalUse(self, word: str) -> bool:
        return (
            word.capitalize() == word or
            word.upper() == word or
            word.lower() == word
        )
        
# @lc code=end

