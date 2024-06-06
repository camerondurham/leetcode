class Solution {
public:
    // beat 96.09% of users with c++
    vector<string> commonChars(vector<string>& words) {
        std::vector<int> prev(26, 0);
        
        bool first = true;
        // [a, b, c, d, e, f, g, ..., z]
        // [1, 2, ]
        // bella = {b, e, l, l, a}
        // label = {l, a, b, e, l}
        // roller = {r, o, l, e}
        for (auto word : words) {
            std::vector<int> cur(26, 0);
            // for each word, create list of chars
            for (char c : word) {
                cur[c - 'a']++;
            }
            if (first) {
                for (int i = 0; i < prev.size(); i++) {
                    prev[i] = cur[i];
                }
            } else {
                for (int i = 0; i < prev.size(); i++) {
                    prev[i] = std::min(prev[i], cur[i]);
                }
            }
            first = false;
        }
        vector<string> result;
        for (int i = 0; i < prev.size(); i++) {
            int count = prev[i];
            while (count > 0) {
                string str(1, ((char) i + 'a'));
                result.push_back(str);
                count--;
            }
        }
        return result;
    }
};
