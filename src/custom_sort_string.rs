use std::collections::HashMap;

struct Solution;

impl Solution {

    pub fn custom_sort_string(order: String, s: String) -> String {
        let n = s.len();

        let mut freqs = HashMap::new();
        for letter in s.chars() {
            freqs
                .entry(letter)
                .and_modify(|c| { *c += 1; })
                .or_insert(1);
        }

        let mut result: Vec<char> = Vec::with_capacity(n);
        for item in order.chars() {
            let has = freqs.contains_key(&item);
            if has {
                let count = freqs[&item];
                for _ in 0..count {
                    result.push(item);
                }
                freqs.remove(&item);
            }
        }
        for (key, value) in freqs {
            for _ in 0..value {
                result.push(key);
            }
        }

        result.iter().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let order = str!("cba");
        let s = str!("abcd");
        let result = Solution::custom_sort_string(order, s);
        assert_eq!(result, "cbad");
    }

    #[test]
    fn example_2() {
        let order = str!("bcafg");
        let s = str!("abcd");
        let result = Solution::custom_sort_string(order, s);
        assert_eq!(result, "bcad");
    }

}
