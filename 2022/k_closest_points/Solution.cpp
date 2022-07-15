//
// Created by cam on 7/15/22.
//

#include "Solution.h"
#include<vector>
#include<algorithm>
#include<functional>
int main() {
    std::vector<std::vector<int>> numbers = {{0, 1}, {1, 3}, {2,0}, {5,3}, {4,9}, {10,10}};
    auto euclidian = [](const std::vector<int>& elem) {
        return (elem[0] * elem[0]) + (elem[1] * elem[1]);
    };
    auto comparator = [=](const std::vector<int>& a, const std::vector<int>& b) {
        return euclidian(a) > euclidian(b);
    };
    std::make_heap(begin(numbers), end(numbers), comparator);
}
