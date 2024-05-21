class Solution {
public:
/*
 * Given a set of distinct integers, nums, return all possible subsets (the power set).
 *  Note: The solution set must not contain duplicate subsets.
 *  Example:
 *  Input: nums = [1,2,3]
 *  Output:
 *  [
 *  [3],
 *  [1],
 *  [2],
 *  [1,2,3],
 *  [1,3],
 *  [2,3],
 *  [1,2],
 *  []
 *  ]
 *  Input: nums = [1,2,3,4]
 */
/*
 *
 * i = current element
 * V = what we're taking powerset of
 * ans = what to return
 * curr = current subset
 * n = size of V
 *
 */
// backtracking solution
void helper(int i, vector<int>& V, vector<vector<int>>& ans, vector<int> curr)
{
    ans.push_back(curr);
    int n = V.size();
    for(int idx = i; idx < n; ++idx)
    {
        curr.push_back(V[idx]);
        helper(idx + 1, V, ans, curr);
        curr.pop_back();
    }
}

vector<vector<int>> subsets(vector<int>& V) {
    vector<vector<int>>  ans;
    helper(0, V, ans, std::vector<int>(0,0));
    return ans;
}

};
