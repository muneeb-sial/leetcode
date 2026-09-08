#
# @lc app=leetcode id=2 lang=python3
#
# [2] Add Two Numbers
#


# @lc code=start
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next


class Solution:
    def addTwoNumbers(
        self, l1: Optional[ListNode], l2: Optional[ListNode]
    ) -> Optional[ListNode]:

        result = ListNode()
        curr = result

        carry = 0
        while l1 or l2 or carry:

            num1 = l1.val if l1 else 0
            num2 = l2.val if l2 else 0

            # sum
            sum = num1 + num2 + carry
            # carry passing
            carry = sum // 10
            val = sum % 10

            curr.next = ListNode(val=val)

            # update pointer
            curr = curr.next
            l2 = l2.next if l2 else None
            l1 = l1.next if l1 else None

        return result.next


# @lc code=end
