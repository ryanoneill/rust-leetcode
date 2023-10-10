use std::collections::HashSet;

/// Every valid email consists of a local name and a domain name, separated by the `@` sign.
/// Besides lowercase letters, the email may contain one or more `'.'` or `'+'`.
///
/// * For example, in `"alice@leetcode.com"`, `"alice"` is the local name, and `"leetcode.com"` is
/// the domain name.
///
/// If you add periods `'.'` between some characters in the local name part of an email address,
/// mail sent there will be forwarded to the same address without dots in the local name. Note that
/// this rule does not apply to domain names.
///
/// * For example, `"alice.z@leetcode.com"` and `"alicez@leetcode.com"` forward to the same email
/// address.
///
/// If you add a plus `'+'` in the local name, everything after the first plus sign will be
/// ignored. This allows certain emails to be filtered. Note that this rule does not apply to dmain
/// names. 
///
/// * For example, `"m.y+name@email.com"` will be forwarded to `"my@email.com"`.
///
/// It is possible to use both of these rules at the same time.
///
/// Given an array of strings `emails` where we send one email to each `emails[i]`, return the
/// number of different addresses that actually receive mails.
struct Solution;

impl Solution {

    // State 0 = Local Name
    // State 1 = Plus Part of Local Name
    // State 2 = Domain Name
    fn normalize(email: String) -> String {
        let mut result = String::from("");
        let mut state = 0;

        for letter in email.chars() {
            if state == 0 { // Before at
                if letter == '+' {
                    state = 1;
                } else if letter == '.' {
                    // Skip It
                } else if letter == '@' {
                    state = 2;
                    result.push(letter);
                } else {
                    result.push(letter);
                }
            } else if state == 1 {
                if letter == '@' {
                    state = 2;
                    result.push(letter);
                } // else Skip It
            } else {
                result.push(letter);
            }
        }

        result
    }

    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut seen = HashSet::new();

        for email in emails {
            let normalized = Self::normalize(email);
            if !seen.contains(&normalized) {
                seen.insert(normalized);
            }
        }

        seen.len() as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let emails = vec![
            "test.email+alex@leetcode.com".to_string(),
            "test.e.mail+bob.cathy@leetcode.com".to_string(),
            "testemail+david@lee.tcode.com".to_string()
        ];
        let result = Solution::num_unique_emails(emails);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let emails = vec![
            "a@leetcode.com".to_string(),
            "b@leetcode.com".to_string(),
            "c@leetcode.com".to_string(),
        ];
        let result = Solution::num_unique_emails(emails);
        assert_eq!(result, 3);
    }

}

