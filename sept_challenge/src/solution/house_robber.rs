struct Solution {}

impl Solution {
    pub fn rob0(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        if nums.len() == 0usize {
            return 0;
        }
        if nums.len() == 1usize {
            return nums[0];
        }
        if nums.len() == 2usize {
            return max(nums[0], nums[1]);
        }

        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        dp[1] = max(nums[0], nums[1]);
        for i in 2..nums.len() {
            dp[i] = max(dp[i - 2] + nums[i], dp[i - 1]);
        }

        return dp[nums.len() - 1];
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let len = nums.len();
        match len {
            0 => 0,
            1 => nums[0],
            2 => max(nums[0], nums[1]),
            _ => {
                let mut dp = vec![0; nums.len()];
                dp[0] = nums[0];
                dp[1] = max(nums[0], nums[1]);
                for i in 2..nums.len() {
                    dp[i] = max(dp[i - 2] + nums[i], dp[i - 1]);
                }
                dp[nums.len() - 1]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
