struct Solution {}

use std::collections::BTreeSet;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if nums.len() == 0 || k <= 0 {
            return false;
        }
        let mut set: BTreeSet<i64> = BTreeSet::new();
        for i in 0..nums.len() {
            let lo = (*nums.get(i).unwrap() as i64) - t as i64;
            let hi = (*nums.get(i).unwrap() as i64) + t as i64;
            println!("lo: {}, hi: {}", lo, hi);
            if !set.is_empty() && lo <= hi && set.range(lo..hi + 1).count() > 0 {
                return true;
            }

            set.insert(nums[i] as i64);

            if i >= k as usize {
                set.remove(&(*nums.get(i - k as usize).unwrap() as i64));
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*

    t=diff between values
    k=diff between indices

    Example 1:

    Input: nums = [1,2,3,1], k = 3, t = 0
    a[0], a[3]
    [1,2,3,1] -> [1,1,2,3]

    Output: true

    Example 2:

    Input: nums = [1,0,1,1], k = 1, t = 2
    [1,0,1,1]
    [0,1,1,1]
    Output: true

    Example 3:

    Input: nums = [1,5,9,1,5,9], k = 2, t = 3
    [1,5,9,1,5,9]
    [1,1,5,5,9,9]
    Output: false



    */

    #[test]
    fn ex1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::contains_nearby_almost_duplicate(nums, 3, 0), true);
    }

    #[test]
    fn ex2() {
        let nums = vec![1, 0, 1, 1];
        assert_eq!(Solution::contains_nearby_almost_duplicate(nums, 1, 2), true);
    }

    #[test]
    fn ex3() {
        let nums = vec![1, 5, 9, 1, 5, 9];
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(nums, 2, 3),
            false
        );
    }

    #[test]
    fn ex4() {
        let nums = vec![-1, -1];
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(nums, 1, -1),
            false
        );
    }

    #[test]
    fn ex5() {
        let nums = vec![0, 2147483647];
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(nums, 1, 2147483647),
            true
        );
    }
}
