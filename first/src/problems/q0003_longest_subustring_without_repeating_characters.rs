pub struct Solution {}
impl Solution {
    // too slow θ(n^3)
    //pub fn length_of_longest_substring(s: String) -> i32 {
    //    let mut res = 0;
    //    for i in 0..s.len() {
    //        for j in i..s.len() {
    //            if Solution::check_repeatition(&s, i, j) {
    //                res = std::cmp::max(res, j - i + 1);
    //            }
    //        }
    //    }

    //    res as i32
    //}

    //fn check_repeatition(s: &String, start: usize, end: usize) -> bool {
    //    // number of ascii character is 128
    //    let mut chars = [0; 128];
    //    for c in s[start..=end].chars() {
    //        let idx = c as usize;
    //        chars[idx] += 1;
    //        if chars[idx] > 1 {
    //            return false
    //        }
    //    }

    //    true
    //}

    // use sliding window
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut chars = [0; 128];

        let mut left = 0;
        let mut right = 0;

        let mut res = 0;

        while right < s.len() {
            let right_c = s.chars().nth(right).unwrap();
            chars[right_c as usize] += 1;

            while chars[right_c as usize] > 1 {
                let left_c = s.chars().nth(left).unwrap();
                chars[left_c as usize] -= 1;
                left += 1;
            }

            res = res.max(right - left + 1);

            right += 1;
        }

        res as i32
    }
}

mod test {
    #[test]
    fn q0003_longest_subustring_without_repeating_characters() {
        use crate::problems::q0003_longest_subustring_without_repeating_characters::Solution;
        assert_eq!(Solution::length_of_longest_substring(String::from("abcabcbb")), 3); // abc
        assert_eq!(Solution::length_of_longest_substring(String::from("bbbbb")), 1); // b
        assert_eq!(Solution::length_of_longest_substring(String::from("pwwkew")), 3); // wke
        assert_eq!(Solution::length_of_longest_substring(String::from("")), 0); // wke
    }
}
