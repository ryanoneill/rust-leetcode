use crate::union_find::UnionFind;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

/// Given a list of `accounts` where each element `accounts[i]` is a list of strings,
/// where the first element `accounts[i][0]` is a name, and the rest of the elements
/// are emails representing emails of the account.
///
/// Now, we would like to merge these accounts. Two accounts definitely belong to the same person
/// if there is some common email to both accounts. Note that even if two accounts have the same
/// name, they may belong to different people as people could have the same name. A person can have
/// any number of accounts initially, but all of their accounts definitely have the same name.
///
/// After merging the accounts, return the accounts in the following format: the first element of
/// each account is the name, and the rest of the elements are the emails in sorted order. The
/// accounts themselves can be returned in any order.
struct Solution;

impl Solution {

    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut results = Vec::new();

        let n = accounts.len();
        let mut uf = UnionFind::new(n);
        let mut emails: HashMap<String, usize> = HashMap::new();

        for i in 0..n {
            let account = &accounts[i];
            let m = account.len();
            for j in 1..m {
                let email = &account[j];
                if emails.contains_key(email) {
                    let other = emails[email];
                    uf.union(i as i32, other as i32);
                } else {
                    emails.insert(email.clone(), i);
                }
            }
        }

        let mut heaps = Vec::new();

        for _ in 0..n {
            let heap = BinaryHeap::new();
            heaps.push(heap);
        }

        for (email, value) in emails {
            let parent = uf.find(value as i32) as usize;
            heaps[parent].push(Reverse(email));
        }

        for i in 0..n {
            let heap = &mut heaps[i];
            if heap.len() > 0 {
                let mut result = Vec::new();
                result.push(accounts[i][0].clone()); // name
                while heap.len() > 0 {
                    let email = heap.pop().unwrap().0;
                    result.push(email);
                }
                results.push(result);
            }
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let accounts = vec![
            vec![String::from("John"), String::from("johnsmith@mail.com"), String::from("john_newyork@mail.com")],
            vec![String::from("John"), String::from("johnsmith@mail.com"), String::from("john00@mail.com")],
            vec![String::from("Mary"), String::from("mary@mail.com")],
            vec![String::from("John"), String::from("johnnybravo@mail.com")]
        ];
        let results = Solution::accounts_merge(accounts);
        assert_eq!(results, vec![
            vec!["John", "john00@mail.com", "john_newyork@mail.com", "johnsmith@mail.com"],
            vec!["Mary", "mary@mail.com"],
            vec!["John", "johnnybravo@mail.com"]
        ]);
    }

    #[test]
    fn example_2() {
        let accounts = vec![
            vec![String::from("Gabe"), String::from("Gabe0@m.co"), String::from("Gabe3@m.co"), String::from("Gabe1@m.co")],
            vec![String::from("Kevin"), String::from("Kevin3@m.co"), String::from("Kevin5@m.co"), String::from("Kevin0@m.co")],
            vec![String::from("Ethan"), String::from("Ethan5@m.co"), String::from("Ethan4@m.co"), String::from("Ethan0@m.co")],
            vec![String::from("Hanzo"), String::from("Hanzo3@m.co"), String::from("Hanzo1@m.co"), String::from("Hanzo0@m.co")],
            vec![String::from("Fern"), String::from("Fern5@m.co"), String::from("Fern1@m.co"), String::from("Fern0@m.co")]
        ];
        let results = Solution::accounts_merge(accounts);
        assert_eq!(results, vec![
            vec!["Gabe","Gabe0@m.co","Gabe1@m.co","Gabe3@m.co"],
            vec!["Kevin","Kevin0@m.co","Kevin3@m.co","Kevin5@m.co"],
            vec!["Ethan","Ethan0@m.co","Ethan4@m.co","Ethan5@m.co"],
            vec!["Hanzo","Hanzo0@m.co","Hanzo1@m.co","Hanzo3@m.co"],
            vec!["Fern","Fern0@m.co","Fern1@m.co","Fern5@m.co"]
        ]);
    }

}

// 1. Write down the problem ✓
//
// 2. Clarify the problem space ✓
// ** Input: Vec<Vec<String>>
// ** Output: Vec<Vec<String>>
// ** accounts[i][0] is a name
// ** accounts[i][1..] are email addresses
// ** identical email addresses with another account imply the same person and should be merged.
// ** len 1..=1000 for accounts
// ** len 2..=10 for each account
// ** only English letters and valid email addresses
// ** People must have the same name across accounts
//
// 3. Write down the test cases
// ** Input: [['j', 'a@b.com'], ['k', 'b@c.com']] Output: Unchanged
// ** Input: [['j', 'a@b.com'], ['j', 'a@b.com']] Output: [['j', 'a@b.com']]
//
// 4. Describe and write down the algorithm
