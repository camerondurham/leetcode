/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func removeLeafNodes(root *TreeNode, target int) *TreeNode {
    if (root == nil) {
        return nil
    }
    if (isLeafNode(root) && root.Val == target) {
        return nil
    }
    var left = removeLeafNodes(root.Left, target)
    var right = removeLeafNodes(root.Right, target)
    root.Left=left
    root.Right=right
    if (isLeafNode(root) && root.Val == target) {
        return nil
    }
    return root
    
}

func isLeafNode(root *TreeNode) bool {
    return root.Left==nil && root.Right==nil
}
