#include <stddef.h>
#include <iostream>

// Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
  ListNode(int x, ListNode *n) : val(x), next(n) {}
};

class Solution {
public:
  bool hasCycle(ListNode *head) {
    if (head == NULL || head->next == NULL) return false;

    ListNode *one_step = head;
    ListNode *two_step = head;

    while (true) {
      if (one_step->next == NULL) {
        return false;
      } else {
        one_step = one_step->next;
      }

      if (two_step->next == NULL || two_step->next->next == NULL) {
        return false;
      } else {
        two_step = two_step->next->next;
      }

      if (one_step == two_step) {
        return true;
      }
    }
  }
};

void assert_eq(bool expected, bool actual) {
  if (expected == actual) {
    std::cout << "ok" << "\n";
  } else {
    std::cout << "ng. expected: " << expected << ", acutal: " << actual << "\n";
  }
}

// void print_list_node(ListNode *head) {
//   ListNode *node = head;
//   while (node != NULL) {
//     std::cout << "node_val: " << node->val << "\n";
//     node = node->next;
//   }
// }

int main() {
  Solution *sol = new Solution;

  // test1: loop
  ListNode node4_1 = ListNode(4);
  ListNode node3_1 = ListNode(3, &node4_1);
  ListNode node2_1 = ListNode(2, &node3_1);
  ListNode node1_1 = ListNode(1, &node2_1);
  (&node4_1)->next = &node2_1;

  assert_eq(true, sol->hasCycle(&node1_1));

  // print_list_node(&node1_1);
  //
  // test2: no loop
  ListNode node4_2 = ListNode(4);
  ListNode node3_2 = ListNode(3, &node4_2);
  ListNode node2_2 = ListNode(2, &node3_2);
  ListNode node1_2 = ListNode(1, &node2_2);

  assert_eq(false, sol->hasCycle(&node1_2));

  assert_eq(false, sol->hasCycle(NULL));
}
