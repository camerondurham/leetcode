struct Solution {}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer = Vec::<Vec<i32>>::new();
        let mut begin = new_interval[0];
        let mut end = new_interval[1];
        for i in 0..intervals.len() {
            if intervals[i][1] < begin {
                answer.push(intervals[i].clone());
            } else if intervals[i][0] > end {
                answer.push(vec![begin, end]);
                begin = intervals[i][0];
                end = intervals[i][1];
            } else if intervals[i][0] <= end || intervals[i][1] >= begin {
                begin = std::cmp::min(begin, intervals[i][0]);
                end = std::cmp::max(end, intervals[i][1]);
            }
        }

        answer.push(vec![begin, end]);

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        // Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
        // Output: [[1,2],[3,10],[12,16]]
        // Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![[1, 2], [3, 10], [12, 16]]
        );
    }
}
