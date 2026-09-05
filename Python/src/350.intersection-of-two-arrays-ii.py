#
# @lc app=leetcode id=350 lang=python3
#
# [350] Intersection of Two Arrays II
#


# @lc code=start
class Solution:
    def intersect(self, nums1: List[int], nums2: List[int]) -> List[int]:
        from collections import Counter

        freq1 = Counter(nums1)
        freq2 = Counter(nums2)

        intersection = freq1.keys() & freq2.keys()

        result = []
        for i in intersection:
            arr = [i] * min(freq1[i], freq2[i])
            result.extend(arr)
        # return list((Counter(nums1) & Counter(nums2)).elements()) // This Shortcut also exits 😉. but current one has a runtime of ~1 millsec
        return result


# @lc code=end
