use std::collections::HashMap;
use std::collections::HashSet;

struct Solution;

impl Solution {

    pub fn remove_duplicate_letters(s: String) -> String {
        let letters: Vec<char> = s.chars().collect();
        let n = letters.len();

        let mut stack: Vec<char> = Vec::new();
        let mut seen: HashSet<char> = HashSet::new();
        let mut last: HashMap<char, usize> = HashMap::new();

        for i in 0..n {
            let c = letters[i];
            last
                .entry(c)
                .and_modify(|value| { *value = i; })
                .or_insert(i);
        }

        for i in 0..n {
            let c = letters[i];
            if !seen.contains(&c) {
                loop {
                    if stack.is_empty() {
                        break;
                    } else {
                        let s = stack.len();
                        let peek = stack[s-1];
                        if c < peek && last[&peek] > i {
                            stack.pop();
                            seen.remove(&peek);
                        } else {
                            break;
                        }
                    }
                }
                seen.insert(c);
                stack.push(c);
            }

        }

        stack.iter().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = str!("bcabc");
        let result = Solution::remove_duplicate_letters(s);
        assert_eq!(result, "abc");
    }

    #[test]
    fn example_2() {
        let s = str!("cbacdcbc");
        let result = Solution::remove_duplicate_letters(s);
        assert_eq!(result, "acdb");
    }

}
