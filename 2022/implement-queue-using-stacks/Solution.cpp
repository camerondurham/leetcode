#include<vector>
using std::vector;
class Solution {
  public:
    Solution() { }
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
