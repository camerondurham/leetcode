package main


func firstUniqChar(s string) int {
    m := map[byte]int{}
    for i := 0; i < len(s); i++ {
        m[s[i]]++
    }

    for i := 0; i < len(s); i++ {
        if m[s[i]] == 1 {
            return i;
        }
    }
    return -1;
}

func firstUniqCharOrig(s string) int {
    m := make(map[rune]int)
    for _, c := range s {
        v, ok := m[c]
        if ok {
            m[c] = v + 1;
        } else {
            m[c] = 1;
        }
    }
    for pos, char := range s {
        if m[char] == 1 {
            return pos;
        }
    }
    return -1;
}
