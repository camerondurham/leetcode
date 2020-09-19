struct Solution {}

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let nums = vec![
            12, 23, 34, 45, 56, 67, 78, 89, 123, 234, 345, 456, 567, 678, 789, 1234, 2345, 3456,
            4567, 5678, 6789, 12345, 23456, 34567, 45678, 56789, 123456, 234567, 345678, 456789,
            1234567, 2345678, 3456789, 12345678, 23456789, 123456789,
        ];

        let mut ans = Vec::<i32>::new();
        for num in nums {
            if num >= low && num <= high {
                ans.push(num);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(Solution::sequential_digits(100, 300), vec![123, 234]);
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::sequential_digits(1000, 13000),
            vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::sequential_digits(10, 1000000000),
            vec![
                12, 23, 34, 45, 56, 67, 78, 89, 123, 234, 345, 456, 567, 678, 789, 1234, 2345,
                3456, 4567, 5678, 6789, 12345, 23456, 34567, 45678, 56789, 123456, 234567, 345678,
                456789, 1234567, 2345678, 3456789, 12345678, 23456789, 123456789
            ]
        );
    }
}

/*

Input constraints:

    10 <= low <= high <= 10^9

Example 1:

Input: low = 100, high = 300

Example 2:

Input: low = 1000, high = 13000
Output:


Pseudocode:

let ans = Vec::<i32>::new()
let num = 1
append_num = 2

digit=8,    7
rept= 1,    3
9-7=2 < 1

9-8=1 = 1


*/
