//
// Created by Kevin Koshy on 03/08/23.
//

#ifndef LC141_ARRAYTOLISTNODE_H
#define LC141_ARRAYTOLISTNODE_H

#include <iostream>
using namespace std;

struct Node {
    int data;
    Node* next;
};

void insert(Node** root, int item) {
    Node* temp = new Node;
    Node* ptr;
    temp->data = item;
    temp->next = NULL;

    if(*root == NULL){
        *root = temp;
    }
    else {
        ptr = *root;
        while(ptr->next != NULL){
            ptr = ptr->next;
        }
        ptr->next = temp;
    }
}

void printListNode(Node* root){
    while(root!= NULL){
        cout <<root->data<<" -> ";
        root = root->next;
    }
}

Node* arrayToList(int arr[], int n){
    Node *root = NULL;
    for(int i = 0; i < n; i++){
        insert(&root, arr[i]);
    }
    return root;
}


#endif //LC141_ARRAYTOLISTNODE_H
