struct Solution {}

impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        let mut start: i64 = 1;
        let num = num as i64;
        while start <= num {
            if start == num {
                return true;
            }
            start *= 4;
        }
        false
    }

    pub fn is_power_of_four_alt(mut num: i32) -> bool {
        if num == 0 {
            return false;
        }

        while num != 1 {
            if num % 4 != 0 {
                return false;
            }
            num /= 4;
        }
        return true;
    }

    // https://leetcode.com/explore/challenge/card/august-leetcoding-challenge/549/week-1-august-1st-august-7th/3412/discuss/80460/1-line-C++-solution-without-confusing-bit-manipulations
    pub fn is_power_of_four_no_loops(num: i32) -> bool {
        return num > 0 && (num & (num - 1)) == 0 && (num - 1) % 3 == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::is_power_of_four(16), true);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::is_power_of_four(5), false);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::is_power_of_four(1), true);
    }

    #[test]
    fn ex4() {
        assert_eq!(Solution::is_power_of_four(2147483647), false);
    }

    ////////////////////////////////////////////////////////////
    //              Alt Solution
    ////////////////////////////////////////////////////////////

    #[test]
    fn ex1a() {
        assert_eq!(Solution::is_power_of_four_alt(16), true);
    }

    #[test]
    fn ex2a() {
        assert_eq!(Solution::is_power_of_four_alt(5), false);
    }

    #[test]
    fn ex3a() {
        assert_eq!(Solution::is_power_of_four_alt(1), true);
    }

    #[test]
    fn ex4a() {
        assert_eq!(Solution::is_power_of_four_alt(2147483647), false);
    }
}
