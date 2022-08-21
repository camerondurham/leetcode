/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */
/*
Runtime: 1 ms, faster than 99.13% of Java online submissions for Balanced Binary Tree.
Memory Usage: 44.2 MB, less than 67.60% of Java online submissions for Balanced Binary Tree.
*/
class Solution {
    public boolean isBalanced(TreeNode root) {
        if ((root == null) || (root.left == null && root.right == null)) {
            return true;
        }
        
        int hL = height(root.left);
        int hR = height(root.right);
        if (Math.abs(hL - hR) > 1) {
            return false;
        } else {
            return isBalanced(root.left) && isBalanced(root.right);
        }
    }
    
    private int height (TreeNode root) {
        if (root == null) {
            return 0;
        }
        if (root.left == null && root.right == null) {
            return 1;
        }
        int leftSubtree = 1 + height(root.left);
        int rightSubtree = 1 + height(root.right);
        return Math.max(leftSubtree, rightSubtree);
    }
}
