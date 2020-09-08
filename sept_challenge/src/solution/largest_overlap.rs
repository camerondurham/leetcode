struct Solution {}

impl Solution {
    fn shift_and_count(shift: (i32, i32), a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> i32 {
        let n = a.len() as i32;
        let mut count = 0;
        let mut b_row = 0;

        for r in shift.0..n {
            let mut b_col = 0;
            for c in shift.1..n {
                let av = a.get(r as usize).unwrap().get(c as usize).unwrap_or(&0);
                let bv = b
                    .get(b_row as usize)
                    .unwrap()
                    .get(b_col as usize)
                    .unwrap_or(&0);
                if av == &1 && bv == &1 {
                    count += 1;
                }
                b_col += 1;
            }
            b_row += 1;
        }
        count
    }

    pub fn largest_overlap(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        let n = a.len() as i32;
        let mut count = 0;
        for row_shift in 0..n {
            for col_shift in 0..n {
                count = std::cmp::max(
                    count,
                    Solution::shift_and_count((row_shift, col_shift), &a, &b),
                );
                count = std::cmp::max(
                    count,
                    Solution::shift_and_count((row_shift, col_shift), &b, &a),
                );
            }
        }
        count
    }
}

use std::collections::HashMap;
use std::iter::FromIterator;

struct Solution2 {}
impl Solution2 {
    pub fn largest_overlap(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        let a_set: Vec<(i32, i32)> = Vec::from_iter(a.iter().enumerate().flat_map(|(ir, row)| {
            row.iter()
                .enumerate()
                .filter(|(ic, c)| **c == 1)
                .map(move |(ic, c)| (ir as i32, ic as i32))
        }));

        let b_set: Vec<(i32, i32)> = Vec::from_iter(b.iter().enumerate().flat_map(|(ir, row)| {
            row.iter()
                .enumerate()
                .filter(|(ic, c)| **c == 1)
                .map(move |(ic, c)| (ir as i32, ic as i32))
        }));

        let mut res = HashMap::new();

        for a_p in &a_set {
            for b_p in &b_set {
                res.entry((a_p.0 - b_p.0, a_p.1 - b_p.1))
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            }
        }

        *res.values().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::largest_overlap(
                vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
                vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]]
            ),
            3
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution2::largest_overlap(
                vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
                vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]]
            ),
            3
        );
    }
}

/*


vec![
vec![1,1,0],
vec![0,1,0],
vec![0,1,0]],
vec![
vec![0,0,0],
vec![0,1,1],
vec![0,0,1]]

shift   r: -1 --> start row 1 on b
        r: +1 --> start row 1 on a

1 1 0   0 0 0
0 1 0   0 1 1
0 1 0   0 0 1


Input: A = [[1,1,0],
            [0,1,0],
            [0,1,0]]
       B = [[0,0,0],
            [0,1,1],
            [0,0,1]]
Output: 3
Explanation: We slide A to right by 1 unit and down by 1 unit.
*/
