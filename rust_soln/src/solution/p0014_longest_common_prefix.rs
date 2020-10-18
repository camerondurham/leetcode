struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {}
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl".to_string()
        );
    }
}
