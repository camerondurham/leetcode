pub struct Solution {}

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut count = 0;
        let mut upper = true;
        let mut lower = true;
        let mut capitalized = true;

        for c in word.chars() {
            upper = upper && c.is_uppercase();
            lower = lower && c.is_lowercase();
            capitalized = capitalized
                && ((count == 0 && c.is_uppercase()) || (count > 0 && c.is_lowercase()));
            count += 1;
        }
        return upper || lower || capitalized;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::detect_capital_use("USA".to_string()), true);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::detect_capital_use("FlaG".to_string()), false);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::detect_capital_use("FLAGs".to_string()), false);
    }

    #[test]
    fn ex4() {
        assert_eq!(Solution::detect_capital_use("Flags".to_string()), true);
    }

    #[test]
    fn ex5() {
        assert_eq!(Solution::detect_capital_use("leetcode".to_string()), true);
    }

    #[test]
    fn ex6() {
        assert_eq!(Solution::detect_capital_use("leetcodE".to_string()), false);
    }
}
