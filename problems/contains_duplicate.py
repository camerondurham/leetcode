class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        # first try to solve with space inefficient solution with a map
        m = dict()
        for num in nums:
            if num in m:
                m[num] += 1
                if m[num] >= 2:
                    return True
            else:
                m[num] = 1
        # skipping early exit case
        for v in m.values():
            if v >= 2:
                return True
        return False

# Time complexity: O(n)
# Space complexity: O(n) since every 

# ------



# Space complexity of O(1)
# Time complexity O(n log n)
class Solution2:
    def containsDuplicate(self, nums: List[int]) -> bool:
        nums.sort()
        for i in range(len(nums) - 1):
            if nums[i] == nums[i+1]:
                return True
        return False

# Best space and runtime 
# Time complexity of O(n)
# Space complexity of O(n)
class Solution3:
    def containsDuplicate(self, nums: List[int]) -> bool:
        s = set()
        for num in nums:
            if num in s:
                return True
            else
                s.add(num)
