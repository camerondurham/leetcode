struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1usize {
            return 0;
        }
        let mut min = prices[0];
        let mut max = 0;
        for i in 1..prices.len() {
            min = std::cmp::min(min, prices[i]);
            max = std::cmp::max(max, prices[i] - min);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        // Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
        //              Not 7-1 = 6, as selling price needs to be larger than buying price.

        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn ex2() {
        // Explanation: In this case, no transaction is done, i.e. max profit = 0.
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
