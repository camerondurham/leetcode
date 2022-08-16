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
      if (head == nullptr || head -> next == nullptr) return nullptr;
      ListNode* fast = head, *slow = head;

      while (fast != nullptr && fast->next != nullptr) {
        fast = fast -> next -> next;
        slow = slow -> next;
        // once loop is found, need to figure out where it starts so begin from the head of the list
        if (fast == slow) {
          ListNode* entry = head;
          while(entry != slow) {
            entry = entry->next;
            slow = slow->next;
          }
          return slow;
        } 

      }
      return nullptr;
    }

    ListNode *detectCycleMemoryHeavy(ListNode *head) {
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
