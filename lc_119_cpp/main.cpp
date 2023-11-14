#include <iostream>
#include <vector>

using namespace std;



class Solution {
public:
    vector<int> getRow(int numRows) {
        vector<vector<int>> triangle;
        triangle.push_back({1});

        for (int rowNum = 1; rowNum <= numRows; rowNum++){
            vector<int>row;
            vector<int>prevRow = triangle[rowNum-1];
            row.push_back({1});

            for (int j = 1; j < rowNum; j++){
                row.push_back(prevRow[j-1]+ prevRow[j]);
            }
            row.push_back({1});
            triangle.push_back(row);
        }

        return triangle.back();

    }
};

int main() {

    Solution result;
    vector<int>ans = result.getRow(3);
//    cout<<ans<<endl;
    return 0;
}
