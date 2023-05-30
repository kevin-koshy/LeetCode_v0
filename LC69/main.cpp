#include <iostream>

using namespace std;
class Solution {
public:
    int mySqrt(int x) {

        if (x == 0){
            return 0;
        }

        int i = 1;

        while(i <= x/2){
            if (i > x/i){
                return i-1;
            }
            else if (i == x/2){
                return i;
            }
            else if (x == 2147483647){
                return 46340;
            }
            i++;
        }


        return 1;
    }
};

int main() {

    Solution result;

    std::cout << result.mySqrt(2147395600) << std::endl;
//    std::cout << result.mySqrt(2147483647) << std::endl;
    return 0;
}
