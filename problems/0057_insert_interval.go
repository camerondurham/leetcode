package main

func Min(i int, j int) int {
	if i < j {
		return i
	}
	return j
}

func Max(i int, j int) int {
	if i > j {
		return i
	}
	return j
}

func insert(intervals [][]int, newInterval []int) [][]int {
	answer := make([][]int, 0)
	for i := 0; i < len(intervals); i++ {
		if intervals[i][1] < newInterval[0] {
			answer = append(answer, intervals[i])
		} else if intervals[i][0] > newInterval[1] {
			answer = append(answer, newInterval)
			newInterval = intervals[i]
		} else if intervals[i][1] >= newInterval[0] || intervals[i][0] <= newInterval[1] {

			newInterval[0] = Min(newInterval[0], intervals[i][0])
			newInterval[1] = Max(newInterval[1], intervals[i][1])
		}
	}
	answer = append(answer, newInterval)

	return answer
}
