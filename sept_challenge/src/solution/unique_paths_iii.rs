struct Solution {}

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut i1 = 0;
        let mut j1 = 0;
        let mut t_steps = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    i1 = i;
                    j1 = j;
                }
                if grid[i][j] != -1 {
                    t_steps += 1;
                }
            }
        }
        Self::dfs(&mut grid.clone(), i1 as i32, j1 as i32, 1, t_steps)
    }

    fn dfs(g: &mut Vec<Vec<i32>>, i: i32, j: i32, start: i32, t_start: i32) -> i32 {
        let ii = i as usize;
        let jj = j as usize;
        if i < 0 || j < 0 || i >= g.len() as i32 || j >= g[0].len() as i32 || g[ii][jj] == -1 {
            return 0;
        }
        if g[ii][jj] == 2 {
            if start == t_start {
                return 1;
            } else {
                return 0;
            }
        }

        // we have now seen this path
        g[ii][jj] = -1;
        let paths = Self::dfs(g, i + 1, j, start + 1, t_start)
            + Self::dfs(g, i - 1, j, start + 1, t_start)
            + Self::dfs(g, i, j + 1, start + 1, t_start)
            + Self::dfs(g, i, j - 1, start + 1, t_start);

        g[ii][jj] = 0;
        paths
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]]),
            2
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]]),
            4
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::unique_paths_iii(vec![vec![0, 1], vec![2, 0]]), 0);
    }
}
