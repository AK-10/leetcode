pub struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        for (i, c1) in s.chars().enumerate() {
            let mut is_contain = false;
            for (j, c2) in s.chars().enumerate() {
                if i == j {
                    continue;
                }
                if c1 == c2 {
                    is_contain = true;
                    break;
                }
            }

            if !is_contain {
                return i as i32;
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use crate::problems::q0387_first_unique_character_in_a_string::Solution;

        let input = "leetcode";
        let expected = 0;
        assert_eq!(Solution::first_uniq_char(input.into()), expected);

        let input = "loveleetcode";
        let expected = 2;
        assert_eq!(Solution::first_uniq_char(input.into()), expected);

        let input = "aabb";
        let expected = -1;
        assert_eq!(Solution::first_uniq_char(input.into()), expected);
    }
}
