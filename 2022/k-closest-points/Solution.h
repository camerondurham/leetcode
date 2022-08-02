//
// Created by cam on 7/15/22.
//

#ifndef LEETCODE_SOLUTION_H
#define LEETCODE_SOLUTION_H

#include<vector>
#include<cmath>
#include<functional>
#include<iostream>

using std::vector;

class Solution {
public:
    vector <vector<int>> kClosest(vector <vector<int>> &points, int k) {
        if (points.size() == k) {
            return points;
        }
        auto euclidian = [](const std::vector<int> &elem) {
            return std::sqrt((elem[0] * elem[0]) + (elem[1] * elem[1]));
        };
        auto comparator = [=](const std::vector<int> &a, const std::vector<int> &b) {
            return euclidian(a) < euclidian(b);
        };

        std::vector <std::vector<int>> soln(points.begin(), points.end());
        std::make_heap(begin(soln), end(soln), comparator);
        while (soln.size() > k) {
            std::pop_heap(soln.begin(), soln.end(), comparator);
            soln.pop_back();
        }
        std::make_heap(begin(soln), end(soln), comparator);
        return soln;

    }

    void print(const std::vector <std::vector<int>> &vec, const std::string &msg) {
        std::cout << msg << '\n';
        for (const auto &outer: vec) {
            for (const auto &inner: outer) {
                std::cout << inner << ',';
            }
            std::cout << '\n';
        }
    }
};


#endif //LEETCODE_SOLUTION_H
