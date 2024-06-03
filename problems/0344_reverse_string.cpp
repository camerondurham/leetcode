class Solution {
public:
    void reverseString(vector<char>& s) {
        // a b c d -> d c b a
        // a b c -> c b a
        int l = 0;
        int r = s.size() - 1;
        while (l < r) {
            char lc = s[l];
            char rc = s[r];
            s[l] = rc;
            s[r] = lc;
            l++;
            r--;
        }
    }
};
