package main
func xorOperation(n int, start int) int {
    res := 0
    for i := 0; i < n * 2; i += 2 {
        res ^= (start + i)
    }
    return res
}
