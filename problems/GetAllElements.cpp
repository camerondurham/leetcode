/**
 * Definition for a binary tree node.
 */
#include<vector>
#include<stack>
using namespace std;

struct TreeNode {
     int val;
     TreeNode *left;
     TreeNode *right;
     TreeNode() : val(0), left(nullptr), right(nullptr) {}
     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 };

class Solution {
public:
    vector<int> getAllElements(TreeNode* root1, TreeNode* root2) {

        vector<int> vec;
        addAll(root1, vec);
        addAll(root2, vec);
        sort(vec.begin(), vec.end());
        return vec;
    }

    void addAll(TreeNode* r, vector<int>& into){
        if (r == nullptr ) {
            return;
        }

        stack<TreeNode*> s;

        s.push(r);

        while(!s.empty()){
            TreeNode* node = s.top();
            s.pop();

            into.push_back(node->val);

            if(node->left) {
                s.push(node->left);
            }

            if(node->right) {
                s.push(node->right);
            }
        }
    }
};
