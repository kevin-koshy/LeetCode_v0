#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int j = 0;
        for (int i = 1; i < size(nums); i++){
            if (nums[j]!= nums[i]){
                j++;
                nums[j] = nums[i];
            }
        }
            return j+1;
    }
};

int main() {
    class result:public Solution{

    };

    std::vector<int>nums{1,2,2,2,3,5,6,7,7,7};
    result r;
    cout<<r.removeDuplicates(nums)<<endl;
    return 0;
}
