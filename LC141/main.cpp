#include <iostream>
#include "../LC2/ArrayToListNode.h"
using namespace std;

//  Definition for singly-linked list.
  struct ListNode {
      int val;
      ListNode *next;
      ListNode(int x) : val(x), next(NULL) {}
  };

class Solution {
public:
    bool hasCycle(Node *head) {
        Node *hare = head;
        Node *tortoise = head;

        while(hare!= NULL && hare->next!=NULL){
            tortoise = tortoise->next;
            hare = hare->next->next;
            if (tortoise == hare) return true;
        }
        return false;
        }
};

int main() {
    Solution result;
    int arr[] = {0,2,3,5,1};
    int n = sizeof(arr)/sizeof(arr[0]);
    Node* root = arrayToList(arr,n);
    printListNode(root);
    cout<<endl;
    cout<<result.hasCycle(root);
    return 0;
}
