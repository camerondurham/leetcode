package main

import "fmt"

func kidsWithCandies(candies []int, extraCandies int) []bool {
	var max_candies = candies[0]
	for _, v := range candies {
		if v > max_candies {
			max_candies = v
		}
	}
	ans := make([]bool, len(candies))
	for i, v := range candies {
		if v+extraCandies >= max_candies {
			ans[i] = true
		}
	}
	return ans
}

func kidsWithCandiesOrig(candies []int, extraCandies int) []bool {
	var max_candies = candies[0]
	for i := 1; i < len(candies); i++ {
		if candies[i] > max_candies {
			max_candies = candies[i]
		}
	}
	ans := make([]bool, len(candies))
	for i := 0; i < len(candies); i++ {
		ans[i] = candies[i]+extraCandies >= max_candies
	}
	return ans
}
