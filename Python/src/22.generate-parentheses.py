#
# @lc app=leetcode id=22 lang=python3
#
# [22] Generate Parentheses
#


# @lc code=start
class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        stack = []
        result = []

        def recurse(open, closed):

            if open < n:
                stack.append("(")
                recurse(open + 1, closed)
                stack.pop()

            if open > closed:
                stack.append(")")
                recurse(open, closed + 1)
                stack.pop()

            if open == closed == n:
                return result.append("".join(stack))

        recurse(0, 0)
        return result


# @lc code=end
