package main

// my solution definitely didn't work
// wasn't careful with making new array

// source: https://leetcode.com/problems/permutations/discuss/120322/golang-version-of-Permutations.-Be-careful-of-the-golang-slice

func permute(nums []int) [][]int {
    var res [][]int
    dfs(&res, nums, 0)
    return res
}

func dfs(res *[][]int, nums []int, start int){
    if start >= len(nums){
        t := make([]int, len(nums))  //be careful of these two lines
        copy(t, nums)
        *res = append(*res, t)
        return
    }
    for i := start; i < len(nums); i++{
        nums[start], nums[i] = nums[i], nums[start]
        dfs(res, nums, start+1)
       nums[start], nums[i] = nums[i], nums[start]
    }
}

