package main
import "sort"

var ans[][]int

func combinationSum(candidates []int, target int) [][]int {
    sort.Ints(candidates)
    ans = make([][]int, 0)
    backtrack(candidates, []int{}, target)
    return ans;
}

func backtrack(nums *[]int, current *[]int,target int) {
    if target == 0 {
        ans = append(ans, *current)
        return
    }
    for i, val := range *nums {
        if val <= target {
            tmp := make([]int, 0, len(*current))
            tmp = append(tmp, *current...)
            tmp = append(tmp, val)
            backtrack((*nums)[i:], &tmp, target - val)
        } else {
            break
        }
    }
}

