struct Solution {}

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answers: Vec<Vec<i32>> = Vec::new();
        Solution::helper(&mut nums, 0, &mut answers);
        answers
    }

    fn helper(nums: &mut Vec<i32>, start: usize, answers: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            answers.push(nums.to_vec());
            return;
        }
        for i in start..nums.len() {
            nums.swap(i, start);
            Solution::helper(nums, start + 1, answers);
            nums.swap(i, start);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}

pub fn assert_eq_vecs(actual: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
    assert_eq!(actual.len(), expected.len());
    let mut actual = actual;
    let mut expected = expected;
    actual.sort();
    expected.sort();
    for a in &mut actual {
        a.sort();
    }
    for a in &mut expected {
        a.sort();
    }
    assert_eq!(actual, expected);
}

fn main() {
    assert_eq_vecs(
        Solution::permute(vec![1, 2, 3]),
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ],
    );
}
