#include<vector>
#include<string>
using namespace std;

class Solution {
public:
    int regionsBySlashes(vector<string>& grid) {

        // transform, must a graph G[N * 3][N * 3]
        /*
            \/
            /\
            ==>
            10 01
            01 10
            01 10
            10 01

            //
            /
            ==>
            01 01
            10 10
            01 00
            10 00
        */
        vector<vector<int>> graph(grid.size() * 3, vector<int>(grid.size() * 3, 0));
        for (int i = 0; i < grid.size(); ++i){
            for (int j = 0; j < grid.size(); ++j){
                if (grid[i][j] == '/'){
                    graph[3*i + 0][3*j + 2] = 1;
                    graph[3*i + 1][3*j + 1] = 1;
                    graph[3*i + 2][3*j + 0] = 1;
                }
                else if(grid[i][j] == '\\'){
                    graph[3*i + 0][3*j + 0] = 1;
                    graph[3*i + 1][3*j + 1] = 1;
                    graph[3*i + 2][3*j + 2] = 1;
                }
            }
        }


        int regions = 0;
        for(int i = 0; i < graph.size(); ++i){
           for(int j = 0; j < graph.size(); ++j){
               if(graph[i][j] == 0){
                   dfs(i, j, graph);
                   regions += 1;

               }
           }
        }


        return regions;
    }


    void dfs(int i, int j, vector<vector<int>>& graph){
        if(i >= 0 && j >= 0 && i < graph.size() && j < graph.size() && graph[i][j] == 0){
            graph[i][j] = 1;
            dfs(i+1,j, graph);
            dfs(i-1,j, graph);
            dfs(i,j+1, graph);
            dfs(i,j-1, graph);
        }
    }
};

/*

[
 " /",
 "/ "
]


[
 0  [0,0,0, 0,0,1],
 1  [0,0,0, 0,1,0],
 2  [0,0,0, 1,0,0]
 3  [0,0,1, 0,0,0],
 4  [0,1,0, 0,0,0],
 5  [1,0,0, 0,0,0],
]

*/
