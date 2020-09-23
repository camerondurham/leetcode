#include<map>
#include<vector>
using namespace std;
class Solution {
public:
    bool carPooling(vector<vector<int>>& trips, int capacity) {
        map<int,int> timestamps;
        for (auto trip: trips){
            int begin_cap = timestamps[trip[1]] + trip[0];
            timestamps[trip[1]] = begin_cap;

            int end_cap = timestamps[trip[2]] - trip[0];
            timestamps[trip[2]] = end_cap;
        }

        int pass = capacity;
        for (auto stamp_element : timestamps) {
            pass += stamp_element.second;
            if (pass > capacity) {
                return false;
            }
        }
        return true;
    }
};
