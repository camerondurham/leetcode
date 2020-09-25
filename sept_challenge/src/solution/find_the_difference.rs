struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let maps = s.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        let mapt = t.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        for (key, val) in mapt.iter() {
            if maps.contains_key(key) == false || maps.get(key).unwrap() != val {
                return *key;
            }
        }
        '0'
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::find_the_difference("abcd".to_string(), "abcde".to_string()),
            'e'
        );
    }
}
