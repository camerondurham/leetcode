/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode(int x) { val = x; }
 * }
 */
/*
Runtime: 4 ms, faster than 100.00% of Java online submissions for Lowest Common Ancestor of a Binary Search Tree.
Memory Usage: 43 MB, less than 97.04% of Java online submissions for Lowest Common Ancestor of a Binary Search Tree.
*/

class Solution {
    public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
        if (root == p || root == q || root == null) { 
            return root; 
        }
        int minVal = Math.min(p.val, q.val);
        int maxVal = Math.max(p.val, q.val);
        if (root.val > maxVal) {
            return lowestCommonAncestor(root.left,p,q);
        } else if (root.val < minVal) {
            return lowestCommonAncestor(root.right,p,q);
        }
        TreeNode left = lowestCommonAncestor(root.left, p, q);
        TreeNode right = lowestCommonAncestor(root.right, p, q);
        boolean subLeftFound = left == q || left == p;
        boolean subRightFound = right == q || right == p;
        if (subLeftFound && subRightFound) {
            return root;
        }  else if (subLeftFound) {
            return left;
        } else if (subRightFound) {
            return right;
        }
        return null;
    }
}
