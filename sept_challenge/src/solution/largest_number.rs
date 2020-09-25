struct Solution {}
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
        nums.sort_by(|a: &String, b: &String| {
            let mut o1 = String::from(&a[..]);
            let mut o2 = String::from(&b[..]);
            o1.push_str(&b);
            o2.push_str(&a);
            o2.cmp(&o1)
        });

        let mut ans = String::new();
        // remove leading zeros and concatenate answer
        nums.iter().for_each(|i: &String| {
            if ans == "0" {
                ans = i.to_string();
            } else {
                ans.push_str(i);
            }
        });
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn ex1() {
        assert_eq!(Solution::largest_number(vec![10, 2]), "210".to_string());
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::largest_number(vec![3, 30, 34, 5, 9]),
            "9534330".to_string()
        );
    }
}
