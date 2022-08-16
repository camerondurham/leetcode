#include<unordered_set>
using std::unordered_set;
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
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
  
    ListNode *detectCycle(ListNode *head) {
        unordered_set<ListNode*> seen;
        ListNode* tmp = head;

        while(tmp != nullptr) {
            if (seen.find(tmp) == seen.end()){
                seen.insert(tmp);
                tmp = tmp->next;
            } else {
                return tmp;
            }
        }
        return nullptr;
    }
};
