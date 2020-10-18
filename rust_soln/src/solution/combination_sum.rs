struct Solution {}

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut ans = Vec::<Vec<i32>>::new();
        let mut holder = Vec::<i32>::new();
        Self::backtrack(&candidates, &mut holder, &mut ans, 0, 0, target);
        ans
    }

    fn backtrack(
        nums: &Vec<i32>,
        current: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
        index: usize,
        sum: i32,
        target: i32,
    ) {
        if sum == target {
            ans.push(current.to_vec());
            return;
        }
        for i in index as usize..nums.len() {
            if nums[i] > target || nums[i] + sum > target {
                return;
            }
            current.push(nums[i]);
            Self::backtrack(nums, current, ans, i, sum + nums[i], target);
            current.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::combination_sum(vec![2], 1),
            Vec::<Vec<i32>>::new()
        );
    }

    #[test]
    fn ex4() {
        assert_eq!(Solution::combination_sum(vec![2], 2), vec![vec![2]]);
    }

    #[test]
    fn ex5() {
        assert_eq!(Solution::combination_sum(vec![1], 2), vec![vec![1, 1]]);
    }
}
/*
Example 1:

Input: candidates = [2,3,6,7], target = 7
Output: [[2,2,3],[7]]
Explanation:
2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
7 is a candidate, and 7 = 7.
These are the only two combinations.

Example 2:

Input: candidates = [2,3,5], target = 8
Output: [[2,2,2,2],[2,3,3],[3,5]]

Example 3:

Input: candidates = [2], target = 1
Output: []

Example 4:

Input: candidates = [1], target = 1
Output: [[1]]

Example 5:

Input: candidates = [1], target = 2
Output: [[1,1]]



Constraints:

    1 <= candidates.length <= 30
    1 <= candidates[i] <= 200
    All elements of candidates are distinct.
    1 <= target <= 500

*/
