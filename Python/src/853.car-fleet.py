#
# @lc app=leetcode id=853 lang=python3
#
# [853] Car Fleet
#


# @lc code=start
class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        point = [(p, s) for p, s in zip(position, speed)]
        fleet = []
        for p, s in sorted(point, reverse=True):
            fleet.append((target-p) /s)
            if len(fleet) >=2 and fleet[-1] <= fleet[-2]:
                fleet.pop()

        return len(fleet)


# @lc code=end
