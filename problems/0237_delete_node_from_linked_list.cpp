struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(nullptr) {}
};
class Solution {
    // this is obviously a dumb problem since we ignore memory management
public:
    void deleteNode(ListNode* node) {
        node->val = node->next->val;
        if(!node->next->next) {
            node->next = nullptr;
        } else {
            deleteNode(node->next);
        }
    }
};
