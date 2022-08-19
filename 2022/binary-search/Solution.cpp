#include<vector>
using std::vector;
class Solution {
public:
    int search(vector<int>& nums, int target) {
        int low = 0, high = nums.size();
        while(low < high) {
            int mid = (high+low)/2;
            if (nums[mid] == target) {
                return mid;
            } else if (nums[mid] < target) {
                low = mid+1;
            } else {
                high = mid;
            }
        }
        return -1;
    }
};
