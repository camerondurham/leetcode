struct Solution {}

impl Solution {
    pub fn length_of_last_word0(s: String) -> i32 {
        let vec: Vec<&str> = s.trim().split(" ").collect();
        match vec.len() {
            0usize => 0,
            _ => vec[vec.len() - 1].len() as i32,
        }
    }
    // faster implementation
    pub fn length_of_last_word(s: String) -> i32 {
        let mut count = 0;
        let mut has_empty = false;
        for c in s.chars() {
            if c == ' ' {
                has_empty = true;
            } else if has_empty {
                has_empty = false;
                count = 1;
            } else {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::length_of_last_word("Hello".to_string()), 5);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::length_of_last_word("".to_string()), 0);
    }

    #[test]
    fn ex4() {
        assert_eq!(Solution::length_of_last_word("a ".to_string()), 1);
    }
}
