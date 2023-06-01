#include <iostream>

using namespace std;

class Solution {
public:
    void merge(vector<int>& nums1, int m, vector<int>& nums2, int n) {
            for(int j = 0, i=m; j < n; j++){
                nums1[i]=nums2[j];
                i++;
            }
        sort(nums1.begin(), nums1.end());


    }
};

void display(const vector<int>&v){
    for(auto i : v){
        cout<<i<<endl;
    }
}

int main() {



    Solution result;
//    vector<int>nums1{1,2,3,0,0,0};
//    vector<int>nums2{2,5,6};

    vector<int>nums1{1};
    vector<int>nums2{};
    result.merge(nums1,1,nums2,0);
    display(nums1);
    return 0;
}
