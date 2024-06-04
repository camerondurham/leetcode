class Solution {
public:
    int longestPalindrome(string s) {
        // palendrome can be built from equal number of letters
        unordered_map<char, int> countMap;
        for (char c : s) {
            if (countMap.find(c) != countMap.end()) {
                countMap[c]++;
            } else {
                countMap[c] = 1;
            }
        }
        bool isOdd = false;
        int count = 0;
        for (const auto& [key, value] : countMap) {
            int div = (value / 2) * 2;
            count += div;
            isOdd = isOdd || value % 2 == 1;
        }
        // aa bb cc d
        return isOdd ? count + 1 : count;
    }
};
