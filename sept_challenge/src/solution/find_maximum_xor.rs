struct Solution {}
struct Solution0 {}

impl Solution0 {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut maxval = 0;
        for i in 0usize..nums.len() {
            for j in i + 1usize..nums.len() {
                maxval = std::cmp::max(maxval, nums[i] ^ nums[j]);
            }
        }
        maxval
    }
}

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 2],
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut trie: Trie = Default::default();
        for &num in nums.iter() {
            let mut node = &mut trie;
            for i in (0..31).rev() {
                let bit = ((num >> i) & 1) as usize;
                node = node.children[bit].get_or_insert_with(Default::default);
            }
        }
        let mut answer = 0;
        for &num in nums.iter() {
            let mut max = 0;
            let mut node = &trie;
            for i in (0..31).rev() {
                let bit = ((num >> i) & 1) as usize;
                if let Some(n) = &node.children[1 - bit] {
                    max |= 1 << i;
                    node = n;
                } else {
                    node = &node.children[bit].as_ref().unwrap();
                }
            }
            answer = std::cmp::max(answer, max);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::find_maximum_xor(vec![2, 4]), 6);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::find_maximum_xor(vec![0]), 0);
    }

    #[test]
    fn ex4() {
        assert_eq!(
            Solution::find_maximum_xor(vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70]),
            127
        );
    }
}
