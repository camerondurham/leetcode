using namespace std;
#import <vector>

class Solution {
public:
    vector<vector<int>> insert(vector<vector<int>>& intervals, vector<int>& newInterval) {
        vector<vector<int>> answer;
        for(int i = 0; i < intervals.size(); ++i)
        {
            if(intervals[i][1] < newInterval[0])
            {
                answer.push_back(intervals[i]);
            }

            else if(intervals[i][0] > newInterval[1])
            {
                answer.push_back(newInterval);
                newInterval = intervals[i];
            }

            else if(intervals[i][1] >= newInterval[0] || intervals[i][0] <= newInterval[1])
            {
                newInterval[0] = std::min(newInterval[0], intervals[i][0]);
                newInterval[1] = std::max(newInterval[1], intervals[i][1]);
            }

        }
        answer.push_back(newInterval);
        return answer;
    }
};
