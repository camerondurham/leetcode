class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        ltor = [1 for i in range(len(nums))] # hold product of everything to the left
        rtol = [1 for i in range(len(nums))] # product of everything before to the right
        # ltor=[1,2,6,24]
        ltor[0] = nums[0]
        for i in range(1,len(nums)):
            ltor[i] = nums[i]*ltor[i-1]
        rtol[-1] = nums[-1]
        # rtol=[24,24,12,4]
        for i in range(len(nums)-2,0, -1):
            rtol[i] = nums[i]*rtol[i+1]
        answ = [0 for i in range(len(nums))]
        for i in range(len(nums)):
            # TODO: write this in a more idiomatic way
            if i == 0:
                answ[i] = rtol[i+1]
            elif i == len(nums) - 1:
                answ[i] = ltor[i-1]
            else:
                answ[i] = rtol[i+1] * ltor[i-1]
        return answ

# thanks claude
from functools import reduce
from operator import mul

class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        prefix_products = [1] + list(reduce(mul, nums[:i], 1) for i in range(1, len(nums)))
        suffix_products = list(reduce(mul, nums[i+1:], 1) for i in range(len(nums)-1, -1, -1)) + [1]
        return [a * b for a, b in zip(prefix_products, suffix_products)]


# 1. `prefix_products` is a list of cumulative products from the left, e.g., `[1, 1, 2, 6, 24]` for `nums = [1, 2, 3, 4]`. It's built using a list comprehension that calculates the product of all elements up to index `i` using `reduce(mul, nums[:i], 1)`.
#
# 2. `suffix_products` is a list of cumulative products from the right, e.g., `[24, 24, 12, 4, 1]` for `nums = [1, 2, 3, 4]`. It's built similarly, but in reverse order.
#
# 3. The final output is obtained by multiplying the corresponding elements of `prefix_products` and `suffix_products` using a list comprehension `[a * b for a, b in zip(prefix_products, suffix_products)]`.
#
# This approach has a time complexity of O(n) and a space complexity of O(n), as it uses two auxiliary lists to store the prefix and suffix products.

# thanks neetcode
class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        res = [1] * (len(nums))

        prefix = 1
        for i in range(len(nums)):
            res[i] = prefix
            prefix *= nums[i]
        
        postfix = 1
        for i in range(len(nums) - 1, -1, -1):
            res[i] *= postfix
            postfix *= nums[i]
        
        return res

