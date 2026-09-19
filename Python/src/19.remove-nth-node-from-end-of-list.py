#
# @lc app=leetcode id=19 lang=python3
#
# [19] Remove Nth Node From End of List
#


# @lc code=start
# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def removeNthFromEnd(self, head: ListNode | None, n: int) -> ListNode | None:
        dummy = ListNode(0, head)
        node = dummy
        offset = head

        while n > 0 and offset:
            offset = offset.next
            n -= 1

        while offset:
            node = node.next
            offset = offset.next

        node.next = node.next.next

        return dummy.next


# @lc code=end
