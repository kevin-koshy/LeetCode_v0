# Definition for singly-linked list.
from typing import Optional
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

    def __repr__(self):
        return self.data

class LinkedList:
    def __init__(self):
        self.head = None

    def __repr__(self):
        node = self.head
        nodes = []
        while node is not None:
            nodes.append(str(node.val))
            node = node.next
        nodes.append("None")
        return " -> ".join(nodes)

llist  = LinkedList()


first_node = ListNode(1)
llist.head = first_node

second_node = ListNode(2)
third_node = ListNode(3)
first_node.next = second_node
second_node.next = third_node


class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head.next == None:
            return None
        llist1 = LinkedList()
        return llist1;


