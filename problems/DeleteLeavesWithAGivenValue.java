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
 *
 * https://leetcode.com/problems/delete-leaves-with-a-given-value
 */
class Solution {
    public TreeNode removeLeafNodes(TreeNode root, int target) {
        if (root == null) {
            return null;
        }
        if (isLeafNode(root) && root.val == target) {
            return null;
        }
        TreeNode left = removeLeafNodes(root.left, target);
        TreeNode right = removeLeafNodes(root.right, target);
        root.left = left;
        root.right = right;
        if (isLeafNode(root) && root.val == target) {
            return null;
        }
        return root;
    }

    private boolean isLeafNode(TreeNode root) {
        return root.left == null && root.right == null;
    }
}

/*
Questions:
- can we assume anything about the tree's nodes' values to cut down search space or must we assume we must check entire tree? (e.g. L >= N >= R)
- space or time constraints?
- how large can the tree be? -> [1, 3000] -> can we assume it fits in stack memory (e.g. can use recursion)
*/
