#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    vector<vector<int>> findMissingRanges(vector<int>& nums, int lower, int upper) {

            nums.insert(nums.begin(),lower-1);
            nums.insert(nums.end(), upper+1);


//        if (lower == upper){
//            return {};
//        }
//
//        if (nums.size() == 0){
//            return {{lower, upper}};
//        }

        vector<vector<int>> vec1;
        for(int i = 1; i < nums.size();i++){
            vector<int>row;
            if (nums[i] - nums[i-1] > 1){
                row.push_back(nums[i-1]+1);
                row.push_back(nums[i]-1);
                vec1.push_back(row);
            }
        }
        return vec1;
    }
};


int main() {
    vector<int>nums = {-1};
    int lower = -2, upper = -1;
    Solution result;
    result.findMissingRanges(nums, lower, upper);
    return 0;
}
