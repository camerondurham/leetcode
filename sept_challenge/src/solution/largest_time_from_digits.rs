struct Solution {}

/*
Given an array of 4 digits, return the largest 24 hour time that can be made.
The smallest 24 hour time is 00:00, and the largest is 23:59.  Starting from 00:00, a time is larger if more time has elapsed since midnight.
Return the answer as a string of length 5.  If no valid time can be made, return an empty string.
Note:

    A.length == 4
    0 <= A[i] <= 9
*/

/*
class Solution {
    private int max_time = -1;

    public String largestTimeFromDigits(int[] A) {
        this.max_time = -1;
        permutate(A, 0);
        if (this.max_time == -1)
            return "";
        else
            return String.format("%02d:%02d", max_time / 60, max_time % 60);
    }

    protected void permutate(int[] array, int start) {
        if (start == array.length) {
            this.build_time(array);
            return;
        }
        for (int i = start; i < array.length; ++i) {
            this.swap(array, i, start);
            this.permutate(array, start + 1);
            this.swap(array, i, start);
        }
    }

    protected void build_time(int[] perm) {
        int hour = perm[0] * 10 + perm[1];
        int minute = perm[2] * 10 + perm[3];
        if (hour < 24 && minute < 60)
            this.max_time = Math.max(this.max_time, hour * 60 + minute);
    }

    protected void swap(int[] array, int i, int j) {
        if (i != j) {
            int temp = array[i];
            array[i] = array[j];
            array[j] = temp;
        }
    }
}
*/

impl Solution {
    pub fn largest_time_from_digits(mut a: Vec<i32>) -> String {
        let mut answers: Vec<(i32, i32)> = Vec::new();

        Solution::permute(&mut a, 0, &mut answers);

        if let Some(max_time) = answers.iter().max() {
            format!("{:02}:{:02}", max_time.0, max_time.1)
        } else {
            String::new()
        }
    }

    fn permute(orig: &mut Vec<i32>, start: usize, ans: &mut Vec<(i32, i32)>) {
        if start == orig.len() {
            let (h, m) = (orig[0] * 10 + orig[1], orig[2] * 10 + orig[3]);
            if h < 24 && m < 60 {
                ans.push((h, m));
                return;
            }
        }

        for i in start..orig.len() {
            orig.swap(i, start);
            Solution::permute(orig, start + 1, ans);
            orig.swap(i, start);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::largest_time_from_digits(vec![1, 2, 3, 4]),
            "23:41".to_string()
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::largest_time_from_digits(vec![5, 5, 5, 5]),
            "".to_string()
        );
    }
}
