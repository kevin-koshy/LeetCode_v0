#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> plusOne(vector<int>& digits) {
    int numDigits = digits.size()-1;

    for (int i = numDigits; i >= 0; i--){
    if(digits[i]==9){
        digits[i]=0;
    }
    else{
        digits[i]+= 1;
        return digits;
    }
    }
    digits.push_back(0);
    digits[0]=1;
    return digits;
    }
};

void print(vector<int> const &v){
    for(auto const e:  v){
        cout<<e;
    }
}


int main() {

    Solution result;
//    vector<int>vec{6,1,4,5,3,9,0,1,9,5,1,8,6,7,0,5,5,4,3};
    vector<int>vec2{1,5};
//    print(result.plusOne(vec));
    cout<<endl;
    print(result.plusOne(vec2));
    return 0;
}
