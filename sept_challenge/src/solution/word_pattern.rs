use std::collections::HashMap;
struct Solution {}

impl Solution {
    pub fn word_pattern(ptn: String, string: String) -> bool {
        let mut pattern_map: HashMap<&str, &str> = HashMap::new();
        let mut string_map: HashMap<&str, &str> = HashMap::new();
        let split: Vec<&str> = string.split(' ').collect();
        let pattern: Vec<String> = ptn.chars().map(|c| c.to_string()).collect();

        if split.len() != pattern.len() {
            return false;
        }

        for i in 0..split.len() {
            let ltr = &pattern.get(i).unwrap();
            let word = &split.get(i).unwrap();

            let inpatt = pattern_map.contains_key(&ltr.to_string()[..]);
            let instr = string_map.contains_key(*word);

            if inpatt == false && instr == false {
                pattern_map.insert(ltr, word);
                string_map.insert(word, ltr);
            } else if inpatt == true && instr == true {
                if (string_map.get(*word).unwrap() != ltr)
                    || (pattern_map.get(&ltr.to_string()[..]).unwrap() != *word)
                {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
    pub fn word_pattern_alt(pattern: String, str: String) -> bool {
        let words: Vec<&str> = str.split(' ').collect();
        if pattern.len() != words.len() {
            return false;
        }
        let pattern_word: HashMap<char, &str> = pattern.chars().zip(str.split(' ')).collect();
        let word_pattern: HashMap<&str, char> =
            pattern_word.iter().map(|(&k, &v)| (v, k)).collect();

        for (ch, word) in pattern.chars().zip(str.split(' ')) {
            match (word_pattern.get(word), pattern_word.get(&ch)) {
                (Some(&a), Some(&b)) => {
                    if a != ch || b != word {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string()),
            true
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string()),
            false
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()),
            false
        );
    }

    #[test]
    fn ex4() {
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog dog dog dog".to_string()),
            false
        );
    }

    #[test]
    fn ex5() {
        assert_eq!(
            Solution::word_pattern("abbacc".to_string(), "dog cat cat dog cat cat".to_string()),
            false
        );
    }

    #[test]
    fn ex6() {
        assert_eq!(
            Solution::word_pattern("abc".to_string(), "b c a".to_string()),
            true
        );
    }
}

/*

    Given a pattern and a string str, find if str follows the same pattern.

    Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in str.

    Example 1:

    Input: pattern = "abba", str = "dog cat cat dog"
    Output: true

    Example 2:

    Input:pattern = "abba", str = "dog cat cat fish"
    Output: false

    Example 3:

    Input: pattern = "aaaa", str = "dog cat cat dog"
    Output: false

    Example 4:

    Input: pattern = "abba", str = "dog dog dog dog"
    Output: false

    Notes:
    You may assume pattern contains only lowercase letters, and str contains lowercase letters that may be separated by a single space.


*/
