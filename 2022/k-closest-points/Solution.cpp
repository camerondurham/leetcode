//
// Created by cam on 7/15/22.
//

#include "Solution.h"
#include<vector>
#include<algorithm>
#include<functional>
#include <iostream>

int main() {
    // [[3,3],[5,-1],[-2,4]]
    std::vector<std::vector<int>> numbers = {{3,3},{5,-1},{-2,4}};
    auto euclidian = [](const std::vector<int>& elem) {
        return (elem[0] * elem[0]) + (elem[1] * elem[1]);
    };
    auto comparator = [=](const std::vector<int>& a, const std::vector<int>& b) {
        return euclidian(a) > euclidian(b);
    };
    std::make_heap(begin(numbers), end(numbers), comparator);
    int k = 0;
    int n = 2;
//    std::vector<std::vector<int>> soln;
//    for(const auto elem : numbers) {
//        soln.push_back(elem);
//    }
//    std::make_heap(begin(soln), end(soln), comparator);
//    while(soln.size() > n) { soln.pop_back(); }
    Solution solution;
    auto soln = solution.kClosest(numbers, 2);

    for(const auto& item : soln) {
        for (const auto& inner : item) {
            std::cout << inner << ',';
        }
        std::cout << '\n';
    }
}
