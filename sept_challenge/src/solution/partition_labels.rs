use std::collections::HashMap;
struct Solution {}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        // let mut last: HashMap<char, i32> = HashMap::new();
        let mut last = vec![0i32; 26];
        let mut ans: Vec<i32> = Vec::new();
        for (i, c) in s.chars().enumerate() {
            last[(c as usize) - 97usize] = i as i32;
        }

        let vec: Vec<i32> = s.chars().map(|x| x as i32).collect();

        let mut last_end = 0i32;
        let mut start = 0i32;
        let mut index = 0i32;

        while index < vec.len() as i32 {
            let end = std::cmp::max(
                last[(vec.get(index as usize).unwrap() - 97) as usize],
                last_end,
            );
            last_end = end;
            if index == end {
                ans.push(end - start + 1);
                start = end + 1;
            }
            index += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        println!("{}", 'a' as i32);
        assert_eq!(
            Solution::partition_labels("ababcbacadefegdehijhklij".to_string()),
            vec![9, 7, 8]
        );
    }
}

/*

    Greedily choose the smallest partition:

    "abaccbdeffed"

    a=>2
    c=>4
    b=>5

    e=>10
    f=>9
    d=>11

*/
