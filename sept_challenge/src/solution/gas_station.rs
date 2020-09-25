struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        for i in 0..n {
            if gas[i] > cost[i] {
                let x = Self::try_loop(&gas, &cost, i, n);
                if x >= 0 {
                    return x;
                }
            }
        }

        -1
    }

    fn try_loop(gas: &Vec<i32>, cost: &Vec<i32>, start_idx: usize, n: usize) -> i32 {
        let mut tank = gas[start_idx] - cost[start_idx];
        let mut counter = (start_idx + 1) % n;
        while counter != start_idx {
            tank += gas[counter] - cost[counter];
            if tank <= 0 && counter != start_idx - 1 {
                return -1;
            }
            counter = (counter + 1) % n;
        }
        start_idx as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }
}

/*

There are N gas stations along a circular route, where the amount of gas at station i is gas[i].

You have a car with an unlimited gas tank and it costs cost[i] of gas to travel from station i to
its next station (i+1). You begin the journey with an empty tank at one of the gas stations.

Return the starting gas station's index if you can travel around the circuit once in the clockwise
direction, otherwise return -1.

Note:

    If there exists a solution, it is guaranteed to be unique.
    Both input arrays are non-empty and have the same length.
    Each element in the input arrays is a non-negative integer.


Input:
vec![1,2,3,4,5],
vec![3,4,5,1,2]

    -2-2-3 3 3
     4 2 0 3 6

Output: 3



Input:
vec![2,3,4],
vec![3,4,3]

Output: -1

*/
