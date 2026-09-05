#
# @lc app=leetcode id=345 lang=python3
#
# [345] Reverse Vowels of a String
#


# @lc code=start
class Solution:
    def reverseVowels(self, s: str) -> str:
        l, r = 0, len(s) - 1
        vowels = {"a", "e", "i", "o", "u", "A", "E", "I", "O", "U"}
        chars = list(s)

        while l < r:
            if chars[l] in vowels and chars[r] in vowels:
                chars[l], chars[r] = chars[r], chars[l]
                l += 1
                r -= 1
                continue

            if chars[l] not in vowels and chars[r] in vowels:
                l += 1
                continue

            if chars[l] in vowels and chars[r] not in vowels:
                r -= 1
                continue
            l += 1
            r -= 1
        
        return "".join(chars)
    


# @lc code=end
