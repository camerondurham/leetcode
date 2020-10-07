/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */

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
    TreeNode* insertIntoBST(TreeNode* root, int val) {
        if (root != nullptr){
            if (val > root->val) {
                if (root->right != nullptr){
                    insertIntoBST(root->right, val);
                } else {
                    root->right = new TreeNode(val, nullptr, nullptr);
                }
            } else {
                if (root->left != nullptr){
                    insertIntoBST(root->left, val);
                } else {
                    root->left = new TreeNode(val, nullptr, nullptr);
                }
            }
        } else {
            return new TreeNode(val, nullptr, nullptr);
        }
        return root;
    }
};
