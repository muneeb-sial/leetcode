#
# @lc app=leetcode id=143 lang=python3
#
# [143] Reorder List
#


# @lc code=start
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reorderList(self, head: ListNode | None) -> None:
        """
        Do not return anything, modify head in-place instead.
        """
        s, f = head, head.next

        while f and f.next:
            s = s.next
            f = f.next.next

        sec = s.next
        prev = s.next = None

        while sec:
            temp = sec.next
            sec.next = prev
            prev = sec
            sec = temp

        first, second = head, prev

        while second:
            t1, t2 = first.next, second.next
            first.next = second
            second.next = t1
            first, second = t1, t2


# @lc code=end
