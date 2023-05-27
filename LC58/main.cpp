#include <iostream>
#include <string>

using namespace std;

class Solution {
public:
    int lengthOfLastWord(string s) {

        int k = 0;
        for(int i = s.length()-1; i >= 0; i--){

            if(s.length()==0){
                return 0;
            }
            else{
                if (s[i]==' '){
                    continue;
                }
                else{
                    k++;
                    if ((i==0)||(s[i-1]==' ')){
                        return k;
                    }
                }
            }

        }
        return s.length();
    }
};


int main() {

    Solution result;
    std::cout <<result.lengthOfLastWord("   fly me   to   the moon  ")<< std::endl;
    std::cout <<result.lengthOfLastWord("hello world ")<< std::endl;
    std::cout <<result.lengthOfLastWord("luffy is still joyboy")<< std::endl;
    std::cout <<result.lengthOfLastWord("a ")<< std::endl;
    std::cout <<result.lengthOfLastWord("a")<< std::endl;
    return 0;
}
