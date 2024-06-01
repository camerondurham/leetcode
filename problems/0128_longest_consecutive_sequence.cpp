class Solution {
public:
    int longestConsecutive(vector<int>& nums) {
        // [100,4,200,1,3,2]
        // 
        // 100 -> 101?
        // [___,1, 2, 3, 4, __________, 100, _________, 200]
        // [10, 9, 8, 7, 6]
        unordered_set<int> all_nums;
        for (int i : nums) {
            all_nums.insert(i);
        }
        int max_len = 0;
        for (int i : nums) {
            // check whether this is the start of a sequence
            if (all_nums.find(i-1) != all_nums.end()) {
                continue;
            }
            int cur_len = 1;
            int cur = i;
            // if it is start of a sequence, then count how long it goes
            while (all_nums.find(i+1) != all_nums.end()) {
                cur_len++;
                i++;
            }
            max_len = max(cur_len, max_len);
        }
        return max_len;
    }
};
