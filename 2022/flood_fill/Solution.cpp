#include<iostream>
#include<vector>
using std::vector;
// https://leetcode.com/problems/flood-fill/
class Solution {
public:
    vector<vector<int>> floodFill(vector<vector<int>>& image, int sr, int sc, int color) {
        // handle edge cases, exit early
        if (image.size() < 1 || image.at(0).size() < 1) {
            return image;
        }
        vector<vector<int>> S;

        int startColor = image[sr][sc];

        // reserve more than enough size for whole vector, in case of full flood fill
        S.reserve(image.size() * image.size());

        S.push_back({sr,sc});

        while(!S.empty()) {
            const auto last = S.back();
            int row = last[0];
            int col = last[1];
            S.pop_back();
            if (startColor == image[row][col]) {
                image[row][col] = color;
            }

            // add neighbors
            addNeighbor(row+1, col, color, startColor, image, S);
            addNeighbor(row-1, col, color, startColor, image, S);
            addNeighbor(row, col+1, color, startColor, image, S);
            addNeighbor(row, col-1, color, startColor, image, S);

        }

        return image;
    }

    void addNeighbor(int row, int col, int color, int startColor, const vector<vector<int>>& image, vector<vector<int>>& stack) {
        if (row >= image.size() || col >= image[0].size()) return;
        if (startColor != image[row][col] || color == image[row][col]) return;
        stack.push_back({row,col});
        return;
    }
};

int main() {

    Solution s;

}