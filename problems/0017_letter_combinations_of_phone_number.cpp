#include<vector>
#include<map>
#include<string>
using namespace std;
class Solution {
public:
    inline static map<char,string> comb = {
        {'2', "abc"},
        {'3', "def"},
        {'4', "ghi"},
        {'5', "jkl"},
        {'6', "mno"},
        {'7', "pqrs"},
        {'8', "tuv"},
        {'9', "wxyz"}
    };

    void helper(vector<string>& ans, string& digits, int i, string prefix){
        if(prefix.size() == digits.size() && !prefix.empty()){
            ans.push_back(prefix);
        } else {
            string letters = comb[digits[i]];
            for(char c : letters){
                helper(ans, digits, i+1, prefix + c);
            }
        }
    }

    vector<string> letterCombinations(string digits) {
        vector<string> ans;
        helper(ans, digits, 0, "");
        return ans;
    }
};
