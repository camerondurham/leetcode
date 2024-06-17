class Solution {
    public int maxProfit(int[] prices) {
        int start = 0;
        int end = 1;
        int maxProfit = 0;
        while (end < prices.length) {
            maxProfit = Math.max(maxProfit, prices[end] - prices[start]);
            if (prices[end] < prices[start]) {
                start = end;
            }
            end++;
        }
        return maxProfit;
    }
}
/**
[7,1,5,3,6,4]
[ ,s, , ,e, ]
[7,1,5,3,6,4]
 */
