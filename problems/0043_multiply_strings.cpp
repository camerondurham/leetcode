#include<string>
#include<vector>
using namespace std;
class Solution {
public:
    string multiply(string num1, string num2) {
        vector<string> nums;
        string prefix = "";
        if(num1 == "0" || num2 == "0")
            return "0";

        for(int i = 0; i < num2.size(); ++i){
            multiply_digit(prefix, num1, num2[num2.size() - i - 1], nums);
            prefix += '0';
        }
        return add(nums);
    }
    void multiply_digit(string prefix, string& num, char digit, vector<string>& nums){
        string total = "";
        int carry = 0;
        for(int i = 0; i < num.size(); i++){
            int n1 = num[num.size() - i - 1] - '0';
            int n2 = digit - '0';
            int n = (n1 * n2 + carry) % 10;
            carry = (n1 * n2 + carry) / 10;
            total = (char) ('0' + n) + total;
        }
        if(carry > 0){
            total = (char) (carry + '0') + total;
        }
        total = total + prefix;
        nums.push_back(total);
    }

    string add(vector<string>& nums){
        if(nums.size() == 1) {
            return nums[0];
        }
        int max_len = 0;
        for(string s : nums){
            if(s.size() > max_len)
                max_len = s.size();
        }
        int i = 0;
        int carry = 0;
        string ans = "";
        while(i < max_len){
            int num = 0;
            for(string s : nums){
                if(i < s.size()){
                    num += s[s.size() - i - 1] - '0';
                }
            }
            ans = (char)  ((num + carry) % 10 + '0') + ans;
            carry= (num + carry) / 10;
            i++;
        }
        if(carry > 0){
            ans = (char) (carry + '0') + ans;
        }
        return ans;
    }
};
