/*
// Definition for a Node.
class Node {
    public int val;
    public List<Node> children;

    public Node() {}

    public Node(int _val) {
        val = _val;
    }

    public Node(int _val, List<Node> _children) {
        val = _val;
        children = _children;
    }
};
*/

class Solution {
    public List<Integer> preorder(Node root) {
        List<Integer> list = new ArrayList<>();
        if (root == null) {
            return list;
        }
        list.add(root.val);
        for(Node n : root.children) {
            List<Integer> children = preorder(n);
            list.addAll(children);
        }
        return list;
    }
    public List<Integer> preorderIterative(Node root) {
      List<Integer> list = new ArrayList<>();
      if (root == null) {
        return list;
      }
      Stack<Node> sk = new Stack<Node>();
      sk.push(root);
      while(!sk.empty()) {
        Node n = sk.pop();
        list.add(n.val);
        for (int i = 0; i < n.children.size(); ++i) {
          sk.push(n.children.get(n.children.size() - i - 1));
        }
      }
      return list;
    }
}
}
