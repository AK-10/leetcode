#include <stddef.h>
#include <assert.h>
#include <iostream>

/**
 * Definition for singly-linked list.
 */
struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
  bool hasCycle(ListNode *head) {
    if (head == NULL || head->next == NULL) return false;

    ListNode *fast = head->next->next;
    ListNode *slow = head->next;

    while(fast != NULL && fast->next != NULL) {
      if (fast == slow) {
        return true;
      }
      fast = fast->next->next;
      slow = slow->next;
    }

    return false;
  };
};

// https://qiita.com/arene-calix/items/a08363db88f21c81d351
// gdbの使い方
int main(int argc, char* argv[]) {
  Solution *s = new Solution();
  // test
  ListNode *a = new ListNode(3);
  ListNode *b = new ListNode(2);
  ListNode *c = new ListNode(0);
  ListNode *d = new ListNode(-4);

  a->next = b;
  b->next = c;
  c->next = d;
  d->next = b;
  assert(s->hasCycle(a));
  std::cout << "[3,2,0,-4] ok" << std::endl;


  ListNode *x = new ListNode(1);
  ListNode *y = new ListNode(2);

  x->next = y;
  y->next = x;
  assert(s->hasCycle(x));
  std::cout << "[1,2] ok" << std::endl;

  ListNode *z = new ListNode(1);

  assert(!s->hasCycle(z));
  std::cout << "[1] ok" << std::endl;

  return 0;
}
