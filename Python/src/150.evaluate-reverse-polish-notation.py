#
# @lc app=leetcode id=150 lang=python3
#
# [150] Evaluate Reverse Polish Notation
#


# @lc code=start
class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        stack = []
        valid_ops = {"+", "-", "*", "/"}

        for i in tokens:
            if i in valid_ops:
                a, b = stack.pop(), stack.pop()
                expression = str(b) + i + str(a)
                stack.append(int(eval(expression)))
            else:
                stack.append(int(i))

        return stack[0]


# @lc code=end
