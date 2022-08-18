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
class Solution {
    public List<List<Integer>> levelOrder(TreeNode root) {
        Queue<TreeNode> q = new LinkedList<TreeNode>();
        List<List<Integer>> list = new ArrayList<>();
        if (root == null) { 
            return list; 
        }
        q.add(root);
        while(q.peek()!= null) {
            List<Integer> inner = new ArrayList<>();
            Queue<TreeNode> q2 = new LinkedList(q);
            q.clear();
            while(q2.peek()!= null) {
                TreeNode cur = q2.remove();
                inner.add(cur.val);
                if (cur.left != null) {
                    q.add(cur.left);
                }
                if (cur.right != null) {
                    q.add(cur.right);
                }
            }
            list.add(inner);
        }
        return list;
    }
}
