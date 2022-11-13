use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

pub struct Solution {}
impl Solution {
    // linear
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     for i in 0 ..= nums.len() - 2 {
    //         for j in i + 1 ..= nums.len() - 1 {
    //             if nums[i] + nums[j] == target {
    //                 return vec![i as i32, j as i32]
    //             }
    //         }
    //     }

    //     return vec![]
    // }

    // use hashmap
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut h = HashMap::<i32, i32>::new();

        for i in 0 .. nums.len() {
            let complement = target - nums[i];
            if h.contains_key(&complement) {
                return vec![*h.get(&complement).unwrap(), i as i32]
            }
            h.insert(nums[i], i as i32);
        }

        return vec![]
    }
}

pub struct ChainedHashTable<T> {
    // for t's length don't get too long
    // n <= t.len
    t: Vec<Vec<T>>,
    n: usize // number of items
}

impl <T> ChainedHashTable<T>
where
    T: Hash + PartialEq + Clone
{
    const W: usize = mem::size_of::<usize>() * 8;

    pub fn new() -> ChainedHashTable<T> {
        ChainedHashTable {
            t: vec![],
            n: 0
        }
    }

    pub fn add(&mut self, x: T) -> bool {
        match self.find(&x) {
            Some(_) => false,
            None => {
                if self.n + 1 > self.t.len() {
                    self.resize();
                }
                self.t[ChainedHashTable::hash(&x)].push(x);
                self.n += 1;

                true
            }
        }
    }

    // θ(n_hash(x))
    pub fn remove(&mut self, x: T) -> Option<T> {
        let j = ChainedHashTable::hash(&x);
        for (i, y) in self.t[j].iter().enumerate() {
            if x == *y {
                let item = self.t[j].remove(i);
                self.n -= 1;

                return Some(item)
            }
        }
        None
    }

    // if hash function is good, it put item to t equally.
    // θ(n/t.len) = θ(1)
    // worst case, hash function always return same value, hashmap stores same list.
    // θ(n)
    pub fn find(&self, x: &T) -> Option<T> {
        let j = ChainedHashTable::hash(x);
        for y in self.t[j].iter() {
            if x == y {
                return Some(y.clone())
            }
        }
        None
    }

    fn resize(&mut self) {
        let capacity = match self.t.len() {
            0 => 1,
            x => x * 2
        };
        let new_t: Vec<Vec<T>> = Vec::with_capacity(capacity);
        let old_t = std::mem::replace(&mut self.t, new_t);

        for (i, v) in old_t.into_iter().enumerate() {
            self.t[i] = v;
        }
    }

    // 乗算hash法
    fn hash(x: &T) -> usize {
        let z = rand::random::<u32>() | (1 as u32);
        let hashcode = ChainedHashTable::hashcode(x) as u32;
        let d = 8 as u32; // ?
        let w = 32 as u32;
        //(((z as u128 * hashcode(x) as u128) % ((1 as u128) << Self::W as u128)) >> (Self::W - self.d) as u128) as usize;
        (((z * hashcode) % ((1 as u32) << w)) >> (w - d)) as usize
    }

    fn hashcode(x: &T) -> usize
    where T: Hash {
        let mut hasher = DefaultHasher::new();
        x.hash(&mut hasher);
        hasher.finish() as usize
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn q0001_two_sum_test() {
        use crate::problems::q0001_two_sum::Solution;

        assert_eq!(Solution::two_sum(vec![1, 2], 3), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![1, 3, 5, 4], 7), vec![1, 3]);
        assert_eq!(Solution::two_sum(vec![10, 4, 30, 1], 5), vec![1, 3]);
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    // #[test]
    // fn chained_hashtable_test() {
    //     use crate::problems::q0001_two_sum::ChainedHashTable;
    //     let mut h = ChainedHashTable::<char>::new();
    //     h.add('a');
    //     h.add('b');
    //     h.add('c');

    //     assert_eq!(Some('a'), h.find(&'a'));
    //     assert_eq!(Some('b'), h.find(&'b'));
    //     assert_eq!(Some('c'), h.find(&'c'));
    // }
}

