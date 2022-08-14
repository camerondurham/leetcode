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
    ListNode* reverseList(ListNode* head) {
        if (head == nullptr || head->next == nullptr) {
            return head;
        }
        ListNode * pre = nullptr, *cur = head, *next=nullptr;
        while(cur != nullptr) {
          // set next to the rest of the list
          next = cur->next;
          // set current to the beginning remainder of the rev list
          cur->next = pre;
          // swap prev and curr
          pre = cur;
          // move on to the rest of the list
          cur = next;
        }
        // return that reversed list that was built up
        return pre;
    }
};

