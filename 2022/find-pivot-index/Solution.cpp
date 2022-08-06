#include<vector>
#include<numeric>
#include<iostream>
#include <cassert>

using std::vector;
class Solution {
public:
    /*
   Given an array of integers nums, calculate the pivot index of this array.

The pivot index is the index where the sum of all the numbers strictly to the left of the index is equal to the sum of all the numbers strictly to the index's right.

If the index is on the left edge of the array, then the left sum is 0 because there are no elements to the left. This also applies to the right edge of the array.

Return the leftmost pivot index. If no such index exists, return -1.

 

Example 1:

Input: nums = [1,7,3,6,5,6]
Output: 3
Explanation:
The pivot index is 3.
Left sum = nums[0] + nums[1] + nums[2] = 1 + 7 + 3 = 11
Right sum = nums[4] + nums[5] = 5 + 6 = 11

Example 2:

Input: nums = [1,2,3]
Output: -1
Explanation:
There is no index that satisfies the conditions in the problem statement.

Example 3:

Input: nums = [2,1,-1]
Output: 0
Explanation:
The pivot index is 0.
Left sum = 0 (no elements to the left of index 0)
Right sum = nums[1] + nums[2] = 1 + -1 = 0

 

Constraints:

    1 <= nums.length <= 104
    -1000 <= nums[i] <= 1000

 

Note: This question is the same as 1991: https://leetcode.com/problems/find-the-middle-index-in-array/
     */

  // Runtime: 52 ms, faster than 22.62% of C++ online submissions for Find Pivot Index.
  // Memory Usage: 31.1 MB, less than 87.71% of C++ online submissions for Find Pivot Index.

  int pivotIndex(vector<int>& nums) {
    // find the sum of all elements in the array
    int right = std::accumulate(nums.begin(), nums.end(), 0);
    int left = 0;


    if (right > 0 && nums.size() == 1) { return -1; }

    for (int i = 0; i < nums.size(); ++i) {
      // we will subtract the current number from the array, check if it's equal to the left side;
      right -= nums[i];
      if (left == right) { return i; }
      left += nums[i];
    }
    return -1;
  }
};

void print(std::vector<int>&v) {
  for ( int i : v ) {
    std::cout << i << "," ;
  }
  std::cout << '\n';
}

int main() {
  Solution s;
  // this is not possible
  vector<int> v1 = {1};
  std::cout<< std::endl;
  std::cout << "v1: "; 
  print(v1);
  std::cout << " res: " << s.pivotIndex(v1);
  /* assert(s.pivotIndex(v1) == -1); */

  // only case where pivot is not -1
  vector<int> v2 = {0};
  std::cout<< std::endl;
  std::cout << "v2: "; 
  print(v2);
  std::cout << " res: " << s.pivotIndex(v2);
  /* assert(s.pivotIndex(v2) == 0); */

  // should return -1 since not possible
  vector<int> v3 = {1, 2, 3};
  std::cout<< std::endl;
  std::cout << "v3: "; 
  print(v3);
  std::cout << " res: " << s.pivotIndex(v3);
  /* assert(s.pivotIndex(v3) == -1); */

  // expect 0
  vector<int> v4 = {2, 1, -1};
  std::cout<< std::endl;
  std::cout << "v4: "; 
  print(v4);
  std::cout << " res: " << s.pivotIndex(v4);
  std::cout << std::endl;
  /* assert(s.pivotIndex(v4) == 0); */

  // expect 3
  vector<int> v5 = {1,7,3,6,5,6};
}
