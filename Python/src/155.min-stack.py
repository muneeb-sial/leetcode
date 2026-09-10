#
# @lc app=leetcode id=155 lang=python3
#
# [155] Min Stack
#


# @lc code=start
class MinStack:

    def __init__(self):
        self.stack = []
        self.min_el = float("inf")

    def push(self, value: int) -> None:
        if not self.stack:
            self.stack.append(0)
            self.min_el =value
        else:
            self.stack.append(value - self.min_el)
            self.min_el = min(self.min_el, value)

    def pop(self) -> None:
        if not self.stack:
            return
        pop = self.stack.pop()
        if pop < 0:
            self.min_el = self.min_el - pop
            return

    def top(self) -> int:
        top = self.stack[-1]
        if top > 0:
            return top + self.min_el
        else:
            return self.min_el

    def getMin(self) -> int:
        return self.min_el


# Your MinStack object will be instantiated and called as such:
# obj = MinStack()
# obj.push(value)
# obj.pop()
# param_3 = obj.top()
# param_4 = obj.getMin()
# @lc code=end
