#include <iostream>
using namespace std;


class Solution {
public:
    int climbStairs(int n) {
        int count[n+1];
        count[0]=1;
        count[1]=1;
        for (int i = 2; i <=n; i++){
            count[i]= count[i-1]+count[i-2];
        }
        return count[n];
    }

};


int main() {
    Solution result;
    std::cout << result.climbStairs(45) << std::endl;
    return 0;
}
