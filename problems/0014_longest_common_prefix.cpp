#include<string>
#include<vector>
using namespace std;
class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {

        if((int) strs.size() == 0)
            return "";

        string pfx = strs.front();

        for(int i = 0; i < (int) strs.size(); ++i){
            int j = 0;
            while(strs.at(i).length() && strs.at(i)[j] == pfx[j]){
              ++j;
            }

            if(j==0) return "";

            else  {
                pfx = pfx.substr(0,j);
            }
        }

        return pfx;

    }
};
