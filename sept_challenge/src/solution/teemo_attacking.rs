struct Solution {}
use std::cmp::min;
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut total = 0;
        for i in 0..time_series.len() {
            if i < time_series.len() - 1 {
                total += min(duration, time_series[i + 1] - time_series[i]);
            } else {
                total += duration;
            }
        }
        total
    }
}

impl Solution {
    pub fn alt_find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        time_series
            .into_iter()
            .scan(0, |hi, t| {
                Some(duration + (t - std::mem::replace(hi, t + duration)).min(0))
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(Solution::find_poisoned_duration(vec![1, 4], 2), 4);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::find_poisoned_duration(vec![1, 2], 2), 3);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::find_poisoned_duration(vec![1, 3, 4, 8], 3), 9);
    }
}
