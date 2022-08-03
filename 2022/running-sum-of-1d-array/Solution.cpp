#include<vector>
using std::vector;
class Solution {
public:
    vector<int> runningSum(vector<int>& nums) {
        int s(0);
        vector<int> v;
        for (int num : nums) { s += num; v.push_back(s); }
        return v;

    }
};
