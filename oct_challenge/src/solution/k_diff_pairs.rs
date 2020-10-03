struct Solution {}
use std::collections::HashSet;
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.clone();
        nums.sort_unstable();
        let mut count = 0;
        let mut set = HashSet::<(i32, i32)>::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] <= nums[j] && nums[j] - nums[i] == k {
                    if set.contains(&(nums[i], nums[j])) == false {
                        count += 1;
                        set.insert((nums[i], nums[j]));
                        set.insert((nums[j], nums[i]));
                    }
                }
            }
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(Solution::find_pairs(vec![3, 1, 4, 1, 5], 2), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::find_pairs(vec![1, 2, 3, 4, 5], 1), 4);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::find_pairs(vec![-1, -2, -3], 1), 2);
    }

    #[test]
    fn ex4() {
        assert_eq!(Solution::find_pairs(vec![1, 3, 1, 5, 4], 0), 1);
    }

    #[test]
    fn ex5() {
        assert_eq!(
            Solution::find_pairs(vec![1, 2, 4, 4, 3, 3, 0, 9, 2, 3], 3),
            2
        );
    }
}

/*
Given an array of integers nums and an integer k, return the number of unique k-diff pairs in the array.

A k-diff pair is an integer pair (nums[i], nums[j]), where the following are true:

    0 <= i, j < nums.length
    i != j
    a <= b
    b - a == k


Example 1:

Input: nums = [3,1,4,1,5], k = 2
Output: 2
Explanation: There are two 2-diff pairs in the array, (1, 3) and (3, 5).
Although we have two 1s in the input, we should only return the number of unique pairs.

Example 2:

Input: nums = [1,2,3,4,5], k = 1
Output: 4
Explanation: There are four 1-diff pairs in the array, (1, 2), (2, 3), (3, 4) and (4, 5).

Example 3:

Input: nums = [1,3,1,5,4], k = 0
Output: 1
Explanation: There is one 0-diff pair in the array, (1, 1).

Example 4:

Input: nums = [1,2,4,4,3,3,0,9,2,3], k = 3
Output: 2

Example 5:

Input: nums = [-1,-2,-3], k = 1
Output: 2


*/
