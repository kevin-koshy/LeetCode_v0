# Definition for singly-linked list.
from typing import Optional
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

    def __repr__(self):
        return self.val

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




class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        current = head
        while current and current.next:
            if current.val == current.next.val:
                current.next = current.next.next
            else:
                current = current.next
        return head

    # def __repr__(self):
    #     node = self.head
    #     nodes = []
    #     while node is not None:
    #         nodes.append(str(node.val))
    #         node = node.next
    #     nodes.append("None")
    #     return " -> ".join(nodes)

llist  = LinkedList()
first_node = ListNode(1)
llist.head = first_node
second_node = ListNode(1)
third_node = ListNode(3)
first_node.next = second_node
second_node.next = third_node

result = Solution().deleteDuplicates(llist.head)
print(llist)


