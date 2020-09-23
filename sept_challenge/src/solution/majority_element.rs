struct Solution {}
struct Solution0 {}
use std::collections::HashMap;
impl Solution0 {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let counts: HashMap<i32, i32> = nums.iter().fold(HashMap::new(), |mut acc, n| {
            *acc.entry(*n).or_insert(0) += 1;
            acc
        });
        let m = nums.len() as i32 / 2;
        let mut ans = 0i32;
        for entry in counts.iter() {
            if entry.1 > &m {
                ans = *entry.0;
                break;
            }
        }
        ans
    }
}

impl Solution {
    // using : https://www.cs.utexas.edu/~moore/best-ideas/mjrty/example.html
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 1i32;
        let mut major = nums[0];
        for i in 1..nums.len() {
            if count == 0 {
                count += 1;
                major = nums[i];
            } else if major == nums[i] {
                count += 1;
            } else {
                count -= 1;
            }
        }
        major
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
