class Solution {
public:
    int scoreOfString(string s) {
        // hello = | 'h' - 'e' | + ...
        int acc = 0;
        for (uint i = 1; i < s.size(); i++) {
            acc+= abs(s[i-1]-s[i]);
        }
        return acc;
    }
};
