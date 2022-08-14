class Solution {
public:
    int maxProfit(vector<int>& prices) {
        if ( prices.size() <= 1) {
            return 0;
        }
        // [7,1,5,3,6,4]
        // buy = 1
        // cur = 6
        // profit = 5
        int profit = 0;
        int buy = prices[0];
        for(int i = 1; i < prices.size(); ++i) {
            const int cur = prices[i];

           // check if cur profit is bigger from last time we bought
           if (profit < (cur - buy))
                profit = cur - buy;

            // check if the prices is lower
            if (cur < buy)
                buy = cur;
        }
        return profit;
    }
};
