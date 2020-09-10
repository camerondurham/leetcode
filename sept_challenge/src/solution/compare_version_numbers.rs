struct Solution {}

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1 = version1
            .split('.')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>();
        let mut v2 = version2
            .split('.')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>();

        while let Some(&0) = v1.last() {
            v1.pop();
        }

        while let Some(&0) = v2.last() {
            v2.pop();
        }

        match v1.cmp(&v2) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(2 + 2, 4);
    }
}
