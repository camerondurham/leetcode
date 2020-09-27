struct Solution {}
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 3 {
            return nums;
        }

        let mut count1 = 0;
        let mut count2 = 0;
        let mut m1 = 0;
        let mut m2 = 1;

        for n in &nums {
            if m1 == *n {
                count1 += 1;
            } else if m2 == *n {
                count2 += 1;
            } else if count1 == 0 {
                count1 = 1;
                m1 = *n;
            } else if count2 == 0 {
                count2 = 1;
                m2 = *n;
            } else {
                count1 -= 1;
                count2 -= 1;
            }
        }

        let mut count1 = 0;
        let mut count2 = 0;

        for n in &nums {
            if *n == m1 {
                count1 += 1;
            } else if *n == m2 {
                count2 += 1;
            }
        }
        let mut ans = Vec::new();
        if count1 > (nums.len() as i32 / 3) {
            ans.push(m1);
        }
        if count2 > (nums.len() as i32 / 3) {
            ans.push(m2);
        }

        ans
    }
}

/*

[1,2,3,1,2] floor(N/3) == 1

m1=3
c1=0

m2=2
c2=0



*/

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), vec![3]);
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]).sort(),
            vec![1, 2].sort()
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::majority_element(vec![3, 2]).sort(),
            vec![3, 2].sort()
        );
    }
}
