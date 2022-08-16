/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
struct ListNode {
   int val;
   ListNode *next;
   ListNode() : val(0), next(nullptr) {}
   ListNode(int x) : val(x), next(nullptr) {}
   ListNode(int x, ListNode *next) : val(x), next(next) {}
};
class Solution {
public:
    ListNode* middleNode(ListNode* head) {
        if (head == nullptr) return nullptr;
        ListNode* fast = head, *slow = head;
        while(fast != nullptr) {
            fast = fast->next;
            if (fast == nullptr)
                return slow;
            else {
                fast = fast->next;
                slow = slow->next;
            }
        }
        return slow;
    }
};
