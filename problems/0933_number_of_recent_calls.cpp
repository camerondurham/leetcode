#include<vector>
using namespace std;
class RecentCounter {
public:
    RecentCounter() {
        stk.resize(10000);
    }


    int ping(int t) {
        stk[max++] = t;

        while(stk[min] < (t - 3000)) { ++min; }
        return max - min;
    }

    vector<int> stk;
    int min = 0;
    int max = 0;

};

/**
 * Your RecentCounter object will be instantiated and called as such:
 * RecentCounter* obj = new RecentCounter();
 * int param_1 = obj->ping(t);
 */
