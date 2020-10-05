#include<vector>
#include<iostream>
using namespace std;
class Solution {
public:
    int removeCoveredIntervalsOrig(vector<vector<int>>& intervals) {
        sort(intervals.begin( ),
             intervals.end( ),
             [ ](const vector<int>& lhs, const vector<int>& rhs)
             { return lhs[0] < rhs[0] || (lhs[0] == rhs[0] && lhs[1] > rhs[1]); });
        int num_removed = 0;
        for(int i = 0; i < intervals.size(); i++){
            // cout << "[" << intervals[i][0] << "," << intervals[i][1] << "]  ";
            if (i > 0 && (intervals[i][1] <= intervals[i-1][1] ||
                          intervals[i][0] == intervals[i-1][0])) {
                intervals[i-1][1] = std::max(intervals[i-1][1], intervals[i][1]);
                num_removed++;
            }
        }
        return intervals.size() - num_removed;

    }
    int removeCoveredIntervals(vector<vector<int>>& intervals) {
        sort(intervals.begin( ),
             intervals.end( ),
             [&](const vector<int>& lhs, const vector<int>& rhs)
             { return lhs[0] < rhs[0] || (lhs[0] == rhs[0] && lhs[1] > rhs[1]); });
        int total = intervals.size();
        vector<int> current = intervals[0];
        for(int i = 1; i < intervals.size(); i++){
            if (current[1] >= intervals[i][1]){
                total--;
            } else {
                current = intervals[i];
            }
        }
        return total;
    }

    /*
    Sort w/Lambda C++ Example:

sort(intervals.begin( ),
     intervals.end( ),
     [ ]( const vector<int>& lhs, const vector<int>& rhs )
    {
       return lhs[0] < rhs[0];
    }
);
    */
};


/*

[[1,4],[3,6],[2,8]]


Good test case:
[[66672,75156],[59890,65654],[92950,95965],[9103,31953],[54869,69855],[33272,92693],[52631,65356],[43332,89722],[4218,57729],[20993,92876]]

*/
