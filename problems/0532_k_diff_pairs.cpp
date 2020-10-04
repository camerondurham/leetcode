#include<unordered_set>
#include<unordered_map>
#include<string>
#include<vector>
using namespace std;

class Solution {
public:

    int findPairs(vector<int>& nums, int k) {
        if (k < 0) return 0;
        int count = 0;
        unordered_map<int,int> counts;
        for(int num : nums){
            if (k == 0 && counts[num] == 1){
                count++;
            } else if (k > 0 && counts[num] == 0) {
                count += counts.count(num + k) + counts.count(num - k);
            }
            counts[num]++;
        }
        return count;
    }


    int findPairsOriginal(vector<int>& nums, int k) {
        int count = 0;
        unordered_set<string> set;
        for (int i=0; i < nums.size(); ++i){
            for(int j=i+1; j < nums.size(); ++j){

                if  (nums[i] <= nums[j] &&
                    nums[j] - nums[i] == k ) {
                    string a = std::to_string(nums[i]);
                    string b = std::to_string(nums[j]);
                    string s = a + "," + b;
                    if (set.count(s) == 0 ){
                        count++;
                        set.insert(s);
                        string s2 = b + "," + a;
                        set.insert(s2);
                    }

                }
            }
        }
        return count;
    }
};
