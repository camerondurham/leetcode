package main

import "sort"

type byRange [][]int

func (a byRange) Len() int {
	return len(a)
}
func (a byRange) Swap(i, j int) {
	a[i], a[j] = a[j], a[i]
}
func (a byRange) Less(i, j int) bool {
	return a[i][0] < a[j][0] || (a[i][0] == a[j][0] && a[i][1] > a[j][1])
}

func removeCoveredIntervals(intervals [][]int) int {
	sort.Sort(byRange(intervals))
	n := len(intervals)
	total := n
	current := intervals[0]
	for i := 1; i < n; i++ {
		if current[1] >= intervals[i][1] {
			total--
		} else {
			current = intervals[i]
		}
	}
	return total
}

/*
Custom Sort in Go

package main

import (
    "fmt"
    "sort"
)

type byLength []string

func (s byLength) Len() int {
    return len(s)
}
func (s byLength) Swap(i, j int) {
    s[i], s[j] = s[j], s[i]
}
func (s byLength) Less(i, j int) bool {
    return len(s[i]) < len(s[j])
}

func main() {
    fruits := []string{"peach", "banana", "kiwi"}
    sort.Sort(byLength(fruits))
    fmt.Println(fruits)
}
*/
