struct Solution {}
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut a = 0;
        let mut b = 0;
        let mut arr = [0; 10];
        for (sch, gch) in secret.chars().zip(guess.chars()) {
            if sch == gch {
                a += 1;
            } else {
                let i = sch as usize - '0' as usize;
                let j = gch as usize - '0' as usize;
                if arr[i] < 0 {
                    b += 1;
                }
                if arr[j] > 0 {
                    b += 1;
                }
                arr[i] += 1;
                arr[j] -= 1;
            }
        }

        format!("{}A{}B", a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::get_hint("1807".to_string(), "7810".to_string()),
            "1A3B".to_string()
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::get_hint("1123".to_string(), "0111".to_string()),
            "1A1B".to_string()
        );
    }
}

/*
Example 1:

Input: secret = "1807", guess = "7810"

Output: "1A3B"

Explanation: 1 bull and 3 cows. The bull is 8, the cows are 0, 1 and 7.

Example 2:

Input: secret = "1123", guess = "0111"

Output: "1A1B"

Explanation: The 1st 1 in friend's guess is a bull, the 2nd or 3rd 1 is a cow.
*/
