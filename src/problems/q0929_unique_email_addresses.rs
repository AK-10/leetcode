use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut hashset = HashSet::new();

        emails.iter().for_each(|email| {
            hashset.insert(Self::get_local_and_domain(email));
        });

        hashset.len() as i32
    }

    fn get_local_and_domain(email: &String) -> (String, String) {
        // split by @
        let splited = email.split('@').collect::<Vec<&str>>();
        let local_name = splited[0];
        let domain_name = splited[1];

        // get_fixed_localname
        let local_name = Self::fixed_local_name(local_name);

        return (local_name, domain_name.to_string());
    }

    fn fixed_local_name(local_name: &str) -> String {
        let mut ret = String::new();

        for c in local_name.chars() {
            match c {
                '.' => continue,
                '+' => break,
                c => ret.push(c),
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use crate::problems::q0929_unique_email_addresses::Solution;

        let input = vec![
            "test.email+alex@leetcode.com".to_string(),
            "test.e.mail+bob.cathy@leetcode.com".to_string(),
            "testemail+david@lee.tcode.com".to_string(),
        ];
        let expected = 2;
        assert_eq!(Solution::num_unique_emails(input), expected);

        let input = vec![
            "a@leetcode.com".to_string(),
            "b@leetcode.com".to_string(),
            "c@leetcode.com".to_string(),
        ];
        let expected = 3;
        assert_eq!(Solution::num_unique_emails(input), expected);
    }
}
