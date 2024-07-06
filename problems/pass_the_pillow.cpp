// https://leetcode.com/problems/pass-the-pillow/
class Solution {
public:
    int passThePillow(int n, int time) {
        int counter = 0;
        bool increase = true;
        int current = 1;
        while (counter < time) {
            if (increase && current+1 <= n) {
                current++;
            } else if (increase && current+1 > n) {
                increase = false;
                current--;
            } else if (!increase && current-1 >= 1) {
                current--;
            } else if (!increase && current-1 < 1) {
                increase = true;
                current++;
            }
            counter++;
        }
        return current;
    }
    // brute force just keep track of the direction and run the simulation until you get to 1/n
};
