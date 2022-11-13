#[derive(Debug)]
struct KthLargest {
    k: i32,
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
impl KthLargest {
    #[allow(unused)]
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut nums = nums.clone();
        nums.sort();
        Self { k, nums }
    }

    #[allow(unused)]
    fn add(&mut self, val: i32) -> i32 {
        if self.nums.is_empty() || val >= self.nums[self.nums.len() - 1] {
            self.nums.push(val);
        } else {
            for (i, v) in self.nums.iter().enumerate() {
                if *v < val {
                    continue;
                } else {
                    self.nums.insert(i, val);
                    break;
                }
            }
        }

        let idx = self.nums.len() - self.k as usize;
        self.nums[idx]
    }
}

// 2, 4, 5, 8 <- 3 => 2, 3, 4, 5, 8 | 4
// 2, 3, 4, 5, 8 <- 5 => 2, 3, 4, 5, 5, 8 | 5
// 2, 3, 4, 5, 5, 8 <- 10 => 2, 3, 4, 5, 5, 8, 10 | 5
// 2, 3, 4, 5, 5, 8, 10 <- 9 =>  2, 3, 4, 5, 5, 8, 9, 10 | 8
// 2, 3, 4, 5, 5, 8, 9, 10 <- 4 =>  2, 3, 4, 4, 5, 5, 8, 9, 10 | 8
//

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use crate::problems::q0703_kth_largest_element_in_a_stream::KthLargest;
        let mut kth_largest = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth_largest.add(3), 4);
        assert_eq!(kth_largest.add(5), 5);
        assert_eq!(kth_largest.add(10), 5);
        assert_eq!(kth_largest.add(9), 8);
        assert_eq!(kth_largest.add(4), 8);
    }
}
