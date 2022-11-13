struct Solution {}
impl Solution {
    #[allow(unused)]
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::<char>::with_capacity(128);

        for c in s.chars() {
            match c {
                '(' => stack.push(c),
                ')' => match stack.last() {
                    Some('(') => {
                        stack.pop();
                    }
                    _ => return false,
                },
                '[' => stack.push(c),
                ']' => match stack.last() {
                    Some('[') => {
                        stack.pop();
                    }
                    _ => return false,
                },
                '{' => stack.push(c),
                '}' => match stack.last() {
                    Some('{') => {
                        stack.pop();
                    }
                    _ => return false,
                },
                _ => {}
            }
        };

        return stack.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use crate::problems::q0020_valid_parentheses::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::is_valid("(())".into()), true);
        assert_eq!(Solution::is_valid("(()".into()), false);
        assert_eq!(Solution::is_valid("())".into()), false);
        assert_eq!(Solution::is_valid(")(".into()), false);
        assert_eq!(Solution::is_valid("()[]{}".into()), true);
        assert_eq!(Solution::is_valid("()[]{}".into()), true);
        assert_eq!(Solution::is_valid("([{}])[]{}".into()), true);

    }
}
