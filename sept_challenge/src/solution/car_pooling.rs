struct Solution {}

use std::collections::BTreeMap;
impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut map = BTreeMap::<i32, i32>::new();
        for trip in trips.iter() {
            let begin_cap = *map.entry(trip[1]).or_insert(0) + trip[0];
            map.insert(trip[1], begin_cap);

            let end_cap = *map.entry(trip[2]).or_insert(0) - trip[0];
            map.insert(trip[2], end_cap);
        }

        let mut cur_cap = 0;
        for val in map.values() {
            cur_cap += val;
            if cur_cap > capacity {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 4),
            false
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5),
            true
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 5, 7]], 3),
            true
        );
    }

    #[test]
    fn ex4() {
        assert_eq!(
            Solution::car_pooling(vec![vec![3, 2, 7], vec![3, 7, 9], vec![8, 3, 9]], 11),
            true
        );
    }
}

/*
Example 1:

Input: trips =
(vec![
vec![2,1,5],
vec![3,3,7]],

 4)
Output: false

Example 2:

Input: trips =
(vec![
vec![2,1,5],
vec![3,3,7]]
, 5)
Output: true

Example 3:

Input: trips =

(vec![
vec![2,1,5],
vec![3,5,7]],  3)
Output: true

Example 4:

Input: trips =
(vec![
vec![3,2,7],
vec![3,7,9],
vec![8,3,9]],  11)
Output: true

*/
