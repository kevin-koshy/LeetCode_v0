#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    int singleNumber(vector<int> &nums) {
        sort(nums.begin(), nums.end());
        int i = 0;
        if (nums.size()==1){
            return nums[0];
        }
        while (i < nums.size()) {
            if (nums[i] != nums[i + 1]) {
                return nums[i];
            }
            i = i + 2;
        }
        return nums[i];
    }
};

int main() {
    Solution result;
    vector<int> numbers = {1};
    int k = result.singleNumber(numbers);
    cout << k <<endl;
    return 0;
}
