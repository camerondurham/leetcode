#include<unordered_map>
#include<string>

using namespace std;
class Solution {
public:
    int firstUniqChar(string s) {
        unordered_map<char,pair<int,int>> m;
        for(int i = 0; i < s.size(); i++) {
            m[s[i]].first++;
            m[s[i]].second = i;
        }
        int idx = s.size();
        for(auto& it: m){
            if(it.second.first == 1) { idx = min(idx, it.second.second); }
        }
        return idx == s.size() ? -1 : idx;
    }
    int firstUniqCharOriginal(string s) {
        unordered_map<char,int> m;
        for(char c : s) { m[c]++; }
        for(int i = 0; i < s.size(); i++) { if(m[s[i]] == 1) return i;}
        return -1;
    }
};
