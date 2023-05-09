use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

enum ComparisonResult {
    ComesBefore(char, char),
    Impossible,
    NoInformation,
}

/// There is a new alien language that uses the English alphabet. However, the
/// order among the letters is unknown to you.
///
/// You are given a list of strings `words` from the alien language's
/// dictionary, where the strings in `words` are sorted lexicographically by
/// the rules of this new language.
///
/// Return a string of the unique letters in the new alien language sorted in
/// lexicographically increasing order by the new language's rules. If there is
/// no solution, return `""`. If there are multiple solutions, return any of
/// them.
struct Solution;

impl Solution {

    fn compare_words(before: &str, after: &str) -> ComparisonResult {
        let mut bchars = before.chars();
        let mut achars = after.chars();
        let result;

        loop {
            match (bchars.next(), achars.next()) {
                (Some(b), Some(a)) => {
                    if b != a {
                        result = ComparisonResult::ComesBefore(b, a);
                        break;
                    }
                }
                (Some(_), None) => {
                    result = ComparisonResult::Impossible;
                    break;
                }
                _ => {
                    result = ComparisonResult::NoInformation;
                    break;
                }
            }
        }

        result
    }

    pub fn alien_order(words: Vec<String>) -> String {
        let n = words.len();

        let mut in_degrees = HashMap::new();
        let mut comes_before = HashMap::new();

        // Initiale in_degrees
        for i in 0..n {
            let word = &words[i];
            for c in word.chars() {
                in_degrees
                    .entry(c)
                    .or_insert(0);
            }
        }

        for i in 0..n-1 {
            let before = &words[i];
            let after = &words[i+1];

            let comparison = Self::compare_words(before, after);
            match comparison {
                ComparisonResult::ComesBefore(b, a) => {
                    let inserted = comes_before
                        .entry(b)
                        .or_insert(HashSet::new())
                        .insert(a);
                    if inserted {
                        in_degrees
                            .entry(a)
                            .and_modify(|count| { *count += 1; })
                            .or_insert(1);
                    }

                }
                ComparisonResult::NoInformation => { } // this is ok
                ComparisonResult::Impossible => {
                    return "".to_string()
                }
            }
        }

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        for (k, v) in in_degrees.iter() {
            if *v == 0 {
                queue.push_back(*k);
                seen.insert(*k);
                result.push(*k);
            }
        }

        while !queue.is_empty() {
            let letter = queue.pop_front().unwrap();
            if comes_before.contains_key(&letter) {
                for other in &comes_before[&letter] {
                    if !seen.contains(other) {
                        in_degrees
                            .entry(*other)
                            .and_modify(|count| { *count -= 1; });
                        if in_degrees[other] == 0 {
                            queue.push_back(*other);
                            seen.insert(*other);
                            result.push(*other);
                        }
                    }
                }
            }
        }

        if seen.len() == in_degrees.len() {
            result.iter().collect::<String>()
        } else {
            "".to_string()
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    // t -> f
    // w -> e
    // r -> t
    // e -> r
    // wertf

    #[test]
    fn example_1() {
        let orig = vec!["wrt", "wrf", "er", "ett", "rftt"];
        let words = orig.iter().map(|s| s.to_string()).collect();
        let result = Solution::alien_order(words);
        assert_eq!(result, "wertf");
    }

    #[test]
    fn example_2() {
        let orig = vec!["z", "x"];
        let words = orig.iter().map(|s| s.to_string()).collect();
        let result = Solution::alien_order(words);
        assert_eq!(result, "zx");
    }

    #[test]
    fn example_3() {
        let orig = vec!["z", "x", "z"];
        let words = orig.iter().map(|s| s.to_string()).collect();
        let result = Solution::alien_order(words);
        assert_eq!(result, "");
    }

    #[test]
    fn example_longer() {
        let orig = vec!["z", "zx"];
        let words = orig.iter().map(|s| s.to_string()).collect();
        let result = Solution::alien_order(words);
        // Return any of them. Not sure the ordering between z and x
        assert!(result == "xz" || result == "zx");
    }

    #[test]
    fn same() {
        let orig = vec!["z", "z"];
        let words = orig.iter().map(|s| s.to_string()).collect();
        let result = Solution::alien_order(words);
        assert_eq!(result, "z");
    }

    #[test]
    fn unknown() {
        let orig = vec!["zy", "zx"];
        let words = orig.iter().map(|s| s.to_string()).collect();
        let result = Solution::alien_order(words);
        // Return any of them. y comes before x, but don't know z
        let options = vec!["yzx", "yxz", "zyx"];
        let options: Vec<String> = options.iter().map(|s| s.to_string()).collect();
        assert!(options.contains(&result));
    }

    #[test]
    fn impossible() {
        let orig = vec!["abc", "ab"];
        let words = orig.iter().map(|s| s.to_string()).collect();
        let result = Solution::alien_order(words);
        assert_eq!(result, "");
    }

}
