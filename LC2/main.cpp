#include <iostream>
//#include "ArrayToListNode.h"
using namespace std;


struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {

        auto *dummyHead = new ListNode(0);
        auto *curr = dummyHead;
        int carry = 0;

        while(l1!= nullptr || l2!= nullptr||carry!=0){
            int x = (l1!= nullptr)?l1->val:0;
            int y = (l2!= nullptr)?l2->val:0;
            int sum = x + y + carry;
            carry = sum / 10;
            curr->next = new ListNode(sum %10);
            curr = curr->next;
            l1 = (l1!= nullptr)? l1->next : nullptr;
            l2 = (l2!= nullptr)? l2->next : nullptr;
        }

        return dummyHead->next;
    }
};

int main() {
    Solution result;
    auto *a1 = new ListNode(7);
    auto *a2 = new ListNode(1);
    auto *a3 = new ListNode(8);
    a1->next = a2;
    a2->next = a3;

    auto *b1 = new ListNode(9);
    auto *b2 = new ListNode(2);
    auto *b3 = new ListNode(6);
    b1->next = b2;
    b2->next = b3;

    auto c = result.addTwoNumbers(a1, b1);

    return 0;
}
