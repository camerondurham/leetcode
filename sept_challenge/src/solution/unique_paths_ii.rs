struct Solution {}
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        dp[0][1] = 1;
        for i in 1..=m {
            for j in 1..=n {
                if obstacle_grid[i - 1][j - 1] != 1 {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
        }

        dp[m][n]
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0]
            ]),
            2
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 0, 1, 1],
                vec![0, 0, 1, 0]
            ]),
            0
        );
    }

    #[test]
    fn ex4() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 0, 0, 1],
                vec![0, 1, 0, 0]
            ]),
            2
        );
    }

    #[test]
    fn ex5() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 0, 0, 1],
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0]
            ]),
            8
        );
    }

    #[test]
    fn ex6() {
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1]]), 0);
    }
}

/*

Example 1:

Input:
vec![
  vec![0,0,0],
  vec![0,1,0],
  vec![0,0,0]
]
Output: 2
Explanation:
There is one obstacle in the middle of the 3x3 grid above.
There are two ways to reach the bottom-right corner:
1. Right -> Right -> Down -> Down
2. Down -> Down -> Right -> Right

Hint:

Since we are making use of pre-computed values along the iteration, this becomes a dynamic programming problem.

if obstacleGrid[i][j] is not an obstacle
     obstacleGrid[i,j] = obstacleGrid[i,j - 1]  + obstacleGrid[i - 1][j]
else
     obstacleGrid[i,j] = 0

*/
