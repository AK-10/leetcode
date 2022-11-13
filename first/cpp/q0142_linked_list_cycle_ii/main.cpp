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
  // headからcycleの始まりのノードまでの距離をa
  // cycleの始まりノードからslowとfastが出会うnodeまでの距離をb
  // slowとfastが出会うnodeからcycleの始まりに戻るまでの距離をcとする
  // fastがcycleを循環する回数をnとする
  // slowがfastに出会うまでの距離はa + b
  // fastがslowに出会うまでの距離はa + n(b + c) + b (n >= 1)
  // になる
  // fastはslowより2倍の距離を進むことから以下の等式が成り立つ
  // 2(a + b) = a + n(b + c) + b
  // a = (n - 1)b + nc
  //

  // bとcの関係を見るとcを通る回数が1回多いことになる
  // nに実際の値を当てはめると
  // a = c
  // a = b + 2c
  // a = 2 + 3c
  // slowとfastが出会ったあと、片方のポインタをheadに向け、1つずつ進める
  // 再度slowとfastが出会った場所がcycleの始まりになる
  ListNode *detectCycle(ListNode *head) {
    ListNode *slow = head;
    ListNode *fast = head;

     while (fast != NULL && fast->next != NULL) {
      slow = slow->next;
      fast = fast->next->next;

      // cycle detected
      if (fast == slow) {
        slow = head;
        while (fast != slow) {
          slow = slow->next;
          fast = fast->next;
        }

        return fast;
      }
    }

    return NULL;
  }
};

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
  assert(s->detectCycle(a) == b);
  std::cout << "[3,2,0,-4] ok" << std::endl;


  ListNode *x = new ListNode(1);
  ListNode *y = new ListNode(2);

  x->next = y;
  y->next = x;
  assert(s->detectCycle(x) == x);
  std::cout << "[1,2] ok" << std::endl;

  ListNode *z = new ListNode(1);

  assert(s->detectCycle(z) == NULL);
  std::cout << "[1] ok" << std::endl;

  assert(s->detectCycle(NULL) == NULL);
  std::cout << "[] ok" << std::endl;
  return 0;
}
