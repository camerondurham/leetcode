struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut A = vec![1; n as usize];
        for i in 1..m {
            for j in 1..n {
                A[j as usize] += A[(j - 1) as usize];
            }
        }
        A[(n - 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::unique_paths(7, 3), 28);
    }

    #[test]
    fn ex4() {
        assert_eq!(Solution::unique_paths(3, 3), 6);
    }
}

fn main() {
    assert_eq!(Solution::unique_paths(3, 7), 28);
    assert_eq!(Solution::unique_paths(3, 2), 3);
    assert_eq!(Solution::unique_paths(7, 3), 28);
    assert_eq!(Solution::unique_paths(3, 3), 6);
}
/*
Input: m = 3, n = 7
Output: 28

Example 2:

Input: m = 3, n = 2
Output: 3
Explanation:
From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
1. Right -> Down -> Down
2. Down -> Down -> Right
3. Down -> Right -> Down

Example 3:

Input: m = 7, n = 3
Output: 28

Example 4:

Input: m = 3, n = 3
Output: 6

*/
