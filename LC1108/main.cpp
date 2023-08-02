#include <iostream>
#include <string>

using namespace std;

class Solution {
public:
    string defangIPaddr(string address) {

        string replaceFrom = ".";
        string replaceWith = "[.]";
        for (std::size_t pos = address.find(replaceFrom);pos!= string ::npos;pos=address.find(replaceFrom,pos)){
            address.replace(pos,replaceFrom.size(),replaceWith);
            pos += replaceWith.size();
        }
        return address;
    }
};

int main() {
    Solution result;
    cout << result.defangIPaddr("192.168.1.1");
    return 0;
}
