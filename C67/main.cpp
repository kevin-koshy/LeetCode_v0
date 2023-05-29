#include <iostream>
using  namespace std;

class Solution {
public:
    string addBinary(string a, string b) {
        int carry = 0;
        string result = "";
        int aSize = a.length();
        int bSize = b.length();
        if(aSize > bSize){
            for(int i = 0; i < aSize-bSize; i++){
                b = "0"+ b;
            }
        }
        else if(aSize < bSize){
            for(int i = 0; i < bSize-aSize;i++){
                a = "0"+a;
            }
        }

        for(int i = a.size()-1; i >= 0; i--) {

            if ((a[i] == '0') && (b[i] == '0') && (carry == 0)) {
                result = '0' + result;
                carry = 0;
            } else if ((a[i] == '0') && (b[i] == '1') && (carry == 0)) {
                result = '1' + result;
                carry = 0;
            } else if ((a[i] == '1') && (b[i] == '0') && (carry == 0)) {
                result = '1' + result;
                carry = 0;
            } else if ((a[i] == '1') && (b[i] == '1') && (carry == 0)) {
                result = '0' + result;
                carry = 1;
            } else if ((a[i] == '0') && (b[i] == '0') && (carry == 1)) {
                result = '1' + result;
                carry = 0;
            } else if ((a[i] == '0') && (b[i] == '1') && (carry == 1)) {
                result = '0' + result;
                carry = 1;
            } else if ((a[i] == '1') && (b[i] == '0') && (carry == 1)) {
                result = '0' + result;
                carry = 1;
            } else if ((a[i] == '1') && (b[i] == '1') && (carry == 1)) {
                result = '1' + result;
                carry = 1;
            }
        }

            if ((a[0] == '1') && (b[0] == '1') && (carry == 1)) {
                result = '1' + result;
            }
            else if ((a[0] == '1') && (b[0] == '0') && (carry == 1)) {
                result = '1' + result;
            }
            else if ((a[0] == '0') && (b[0] == '1') && (carry == 1)) {
                result = '1' + result;
            }
            else if ((a[0] == '1') && (b[0] == '1') && (carry == 0)) {
                result = '1' + result;
            }

        return result;
    }
};

void display(const string  & str){
    for (int i = 0; i < str.size(); i++){
        cout<<str[i];
    }
}

int main() {

    setbuf(stdout, 0);
    Solution result;
    display(result.addBinary("1","111"));


    return 0;
}

