struct Solution {}
impl Solution {
    // all combinations of k numbers sum to n (uniq 1..9)
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::<Vec<i32>>::new();
        let mut v = Vec::<i32>::new();
        Self::dfs(1, k as usize, n, &mut v, &mut ans);
        ans
    }

    pub fn dfs(start: i32, k: usize, target: i32, v: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if target == 0 && k == v.len() {
            ans.push(v.to_vec());
            return;
        }

        if k == v.len() {
            return;
        }

        for ii in start..=9 {
            v.push(ii);
            Self::dfs(ii + 1, k, target - ii, v, ans);
            v.pop();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::combination_sum3(3, 7), vec![vec![1, 2, 4]]);
    }
}
