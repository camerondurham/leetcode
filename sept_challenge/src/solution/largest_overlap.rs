struct Solution {}

impl Solution {
    fn shift_and_count(shift: (i32, i32), a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> i32 {
        let n = a.len() as i32;
        let mut count = 0;
        let mut b_row = 0;

        for r in shift.0..n {
            let mut b_col = 0;
            for c in shift.1..n {
                let av = a.get(r as usize).unwrap().get(c as usize).unwrap();
                let bv = b.get(b_row as usize).unwrap().get(b_col as usize).unwrap();
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
                let cur_count = Solution::shift_and_count((row_shift, col_shift), &a, &b);
                let cur_count_swap = Solution::shift_and_count((row_shift, col_shift), &b, &a);
                count = std::cmp::max(count, cur_count);
                count = std::cmp::max(count, cur_count_swap);
            }
        }
        count
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
            Solution::largest_overlap(
                vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0],],
                vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1],]
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
