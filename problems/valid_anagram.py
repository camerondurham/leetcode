class Solution:
    # space complexity O(n)
    # time complexity O(n)
    def isAnagram(self, s: str, t: str) -> bool:
        char_count = dict()
        for i in s:
            if i in char_count:
                char_count[i] += 1
            else:
                char_count[i] = 1
        for i in t:
            if i in char_count:
                char_count[i] -= 1
            else:
                char_count[i] = -1
        for i in char_count.values():
            if i != 0:
                return False
        return True

class Solution2:
    def isAnagram(self, s: str, t: str) -> bool:
        char_count = {}
        for i in s:
            char_count[i] = char_count.get(i, 0) + 1
        for i in t:
            char_count[i] = char_count.get(i, 0) - 1
        for i in char_count.values():
            if i != 0:
                return False
        return True
