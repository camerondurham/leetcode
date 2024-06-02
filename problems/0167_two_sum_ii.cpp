class Solution {
public:
    vector<int> twoSum(vector<int>& numbers, int target) {
        // target = 7
        // [1, 2, 3, 5, 9, 11]
        vector<int> res;
        int l = 0;
        int r = numbers.size() - 1;
        while (l < r) {
            int current = numbers[l] + numbers[r];
            if (current > target) {
                r--;
            } else if (current < target) {
                l++;
            } else {
                res.push_back(l+1);
                res.push_back(r+1);
                return res;
            }
        }
        return res;
    }
};
