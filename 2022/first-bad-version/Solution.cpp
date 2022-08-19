// The API isBadVersion is defined for you.
// bool isBadVersion(int version);

class Solution {
public:
    int firstBadVersion(int n) {
        unsigned int lo = 1, hi = n;
        if (n == 1) { return n; }
        while (lo < hi) {
            int mid = (hi+lo)/2;
            if (isBadVersion(mid)) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        return (hi+lo)/2;
    }
};
