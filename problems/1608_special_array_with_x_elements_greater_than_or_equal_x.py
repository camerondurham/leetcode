class Solution:
    def specialArray(self, nums: List[int]) -> int:
        l = [0] * (len(nums)+1)
        for i in range(len(nums)+1):
            acc = 0
            for j in nums:
                if j >= i:
                    acc+=1
            l[i] = acc
        for x in range(len(l)):
            if x == l[x]:
                return x
        return -1

