struct Solution {}
impl Solution {
    pub fn xor_operation_old(n: i32, start: i32) -> i32 {
        let mut accumulator = start;
        for i in 1..n {
            accumulator = accumulator ^ (start + i * 2)
        }
        accumulator
    }
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        (1..n).fold(start, |acc, i| acc ^ (start + i * 2))
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(2 + 2, 4);
    }
}
