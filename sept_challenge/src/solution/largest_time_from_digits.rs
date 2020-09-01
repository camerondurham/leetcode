struct Solution {}

/*
Given an array of 4 digits, return the largest 24 hour time that can be made.
The smallest 24 hour time is 00:00, and the largest is 23:59.  Starting from 00:00, a time is larger if more time has elapsed since midnight.
Return the answer as a string of length 5.  If no valid time can be made, return an empty string.
Note:

    A.length == 4
    0 <= A[i] <= 9
*/

use std::collections::HashSet;

// 4,3,2,1 => 23:41

// 4,3,2,1 => 23:41
// 4,2,2,2 => 21:42

impl Solution {
    pub fn largest_time_from_digits(a: Vec<i32>) -> String {
        // pack each bucket:
        //  [0:2] [0:9] : [0:5] [0:9]
        let mut hs: HashSet<i32> = HashSet::new();

        for i in a {
            hs.insert(i);
        }

        let mut s = String::new();

        // 1,2,3,4 =>
        String::from("00:00")
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
