#include<vector>
using std::vector;

/**
Runtime: 3 ms, faster than 47.47% of C++ online submissions for Implement Queue using Stacks.
Memory Usage: 6.9 MB, less than 89.70% of C++ online submissions for Implement Queue using Stacks.
*/
class MyQueue {
public:
    MyQueue() {
        
    }
    void push(int x) {
      A.push_back(x);
    }
    int pop() {
      queueify();
      int last = B.back();
      B.pop_back();
      return last;
    }
    int peek() {
      queueify();
      return B.back();
    }
    bool empty() {
        return A.empty() && B.empty();
    }

  private:
    vector<int> A;
    vector<int> B;

    void queueify() {
      if (B.empty()) {
        while(!A.empty()) {
          B.emplace_back(A.back());
          A.pop_back();
        }
      }
    }

};

/**
 * Your MyQueue object will be instantiated and called as such:
 * MyQueue* obj = new MyQueue();
 * obj->push(x);
 * int param_2 = obj->pop();
 * int param_3 = obj->peek();
 * bool param_4 = obj->empty();
 */
