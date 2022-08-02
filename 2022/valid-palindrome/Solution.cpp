//
// Created by cam on 7/10/22.
//

#include "Solution.h"
#include <iostream>
#include <vector>

int main() {
    Solution s;
    std::vector<std::string> vec = {
            "A man, a plan, a canal: Panama",
            "race a car",
            " ",
            "a."
    };

    for (const auto str : vec) {
        std::cout << "input = " << str << " answer = " << s.isPalindrome(str) << "\n";
    }
}