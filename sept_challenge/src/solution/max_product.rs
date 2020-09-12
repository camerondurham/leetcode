use std::cmp;
use std::cmp::{max, min};
use std::mem;
struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut best = nums[0];
        let mut products = vec![(nums[0], 1)];

        for i in 1..nums.len() {
            let (pos, neg) = products[i - 1];

            let b = min(min(neg * nums[i], pos * nums[i]), nums[i]);
            let a = max(max(neg * nums[i], pos * nums[i]), nums[i]);

            products.push((a, b));
            best = max(best, products[i].0);
        }
        best
    }

    // better memory performance
    pub fn max_product_2(nums: Vec<i32>) -> i32 {
        let mut nmax: Vec<i32> = vec![0; nums.len()];
        let mut nmin: Vec<i32> = vec![0; nums.len()];
        nmax[0] = nums[0];
        nmin[0] = nums[0];
        for i in 1..nums.len() {
            let cur = nums[i];
            nmax[i] = max(cur, max(nmax[i - 1] * cur, nmin[i - 1] * cur));
            nmin[i] = min(cur, min(nmax[i - 1] * cur, nmin[i - 1] * cur));
        }
        *nmax.iter().max().unwrap()
    }

    // best memory performance, closest to c++ implementation

    pub fn max_product_3(nums: Vec<i32>) -> i32 {
        let (mut max, mut min, mut ans) = (1, 1, i32::min_value());
        for num in nums {
            if num < 0 {
                mem::swap(&mut max, &mut min);
            }
            max = cmp::max(max * num, num);
            min = cmp::min(min * num, num);
            ans = cmp::max(ans, max);
        }
        ans
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

/*

My CPP solution:

int maxProduct(vector<int>& nums) {
    int best = nums[0];
    for(int i = 1, imin = best, imax = best; i < nums.size(); ++i)
    {
        if(nums[i] < 0)
            std::swap(imin, imax);
        imax = std::max(nums[i], nums[i] * imax);
        imin = std::min(nums[i], nums[i] * imin);

        best = std::max(best, std::max(imax, imin));
    }
    return best;
}
*/
