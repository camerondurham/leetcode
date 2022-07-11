//
// Created by cam on 7/10/22.
//

#ifndef LEETCODE_SOLUTION_H
#define LEETCODE_SOLUTION_H
#include <cctype>
#include <string>
using std::string;

class Solution {
public:
    // Input: s = "A man, a plan, a canal: Panama"
    bool isPalindrome(string s) {
        // get two pointers to either side of the string
        int left = 0;
        int right = s.size() - 1;
        // iterate through the string, finding the next alphanumeric character
        while(left < right) {
            // skip until both are alphanumeric
            while(right >= 1 && !isalphanum(s.at(right))) { right--; }

            while(left < s.size() - 1 && !isalphanum(s.at(left))) { left++; }

            if (lowercase(s[right]) != lowercase(s[left]) && isalphanum(s[left]) && isalphanum(s[right])) {
                return false;
            }

            left++;
            right--;
        }
        return true;
    }

    bool isalphanum(const char c) {
        // make sure it's representable as unsigned char
        // otherwise it's UB
        // https://en.cppreference.com/w/cpp/string/byte/isalpha
        return std::isalpha(static_cast<unsigned char>(c)) || std::isdigit(static_cast<unsigned char>(c));
    }
    char lowercase(const char c) {
        // make sure it's representable as unsigned char
        // otherwise it's UB
        return std::tolower(static_cast<unsigned char>(c));
    }
};

#endif //LEETCODE_SOLUTION_H
