class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        # create a mapping of current number to the difference to get to the target
        # if we can find the "difference", then return the indexes of the those entries
        m = {} # map: {value : index}
        for i in range(len(nums)):
            # diff = 7
            diff = target - nums[i]
            # check if we've seen the difference in our list
            if diff in m:
                return [i, m[diff]]
            else:
                # store the value to index
                m[nums[i]] = i

