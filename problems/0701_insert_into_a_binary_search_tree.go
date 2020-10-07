package main
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */


type TreeNode struct {
     Val int
     Left *TreeNode
     Right *TreeNode
 }

func insertIntoBSTVerbose(root *TreeNode, val int) *TreeNode {
    if(root != nil){
        if(val > root.Val){
            if(root.Left != nil){
                insertIntoBSTVerbose(root.Left, val);
            } else {
                root.Left = new(TreeNode)
                root.Left.Val = val
            }
        } else {
            if(root.Right != nil){
                insertIntoBSTVerbose(root.Right, val);
            } else {
                root.Right = new(TreeNode)
                root.Right.Val = val
            }
        }
    } else {
        rtn := new(TreeNode)
        rtn.Val = val
        return rtn
    }
    return root;
}

func insertIntoBST(root *TreeNode, val int) *TreeNode {
    if root == nil {
        return &TreeNode{Val: val}
    }

    if val > root.Val {
        root.Right = insertIntoBST(root.Right, val)
    } else {
        root.Left = insertIntoBST(root.Left, val)
    }
    return root
}
