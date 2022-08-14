class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        if len(prices) <= 1:
            return 0
        maxP = 0
        lo = 0
        for i in range(1,len(prices)):
            latestDiff = prices[i] - prices[lo]
            maxP = latestDiff if latestDiff > maxP else maxP
            lo = i if prices[i] < prices[lo] else lo

        return maxP

