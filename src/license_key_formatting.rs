/// You are given a license key represented as a string `s` that consists of only alphanumeric
/// characters and dashes. The string is separated into `n + 1` groups by `n` dashes. You are also
/// given an integer `k`.
///
/// We want to reformat the string `s` such that each group contains exactly `k` characters, except
/// for the first group, which could be shorter than `k` but still must contain at least one
/// character. Furthermore, there must be a dash inserted between the two groups, and you should
/// convert all lowercase letters to uppercase.
///
/// Return the reformatted license key.
struct Solution;

impl Solution {

    fn count(str: &str) -> usize {
        let mut result = 0;
        for c in str.chars() {
            match c {
                '-' => {}
                _   => { result += 1; }
            }
        }

        result
    }

    pub fn license_key_formatting(s: String, k: i32) -> String {
        let k = k as usize;
        let n = Self::count(&s);
        let m = n / k;
        let mut first_group_size = n % k;
        if first_group_size == 0 {
            first_group_size = k;
        }
        let mut letters = Vec::with_capacity(n + m);

        let mut is_first = true;
        let mut current = 0;

        for c in s.chars() {
            if c != '-' {
                if is_first {
                    if current == first_group_size {
                        letters.push('-');
                        current = 0;
                        is_first = false;
                    }
                } else if current == k {
                    letters.push('-');
                    current = 0;
                }
                if c.is_ascii_lowercase() {
                    letters.push(c.to_ascii_uppercase());
                } else {
                    letters.push(c);
                }
                current += 1;
            }
        }

        letters.into_iter().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = str!("5F3Z-2e-9-w");
        let k = 4;
        let result = Solution::license_key_formatting(s, k);
        assert_eq!(result, "5F3Z-2E9W")
    }

    #[test]
    fn example_2() {
        let s = str!("2-5g-3-J");
        let k = 2;
        let result = Solution::license_key_formatting(s, k);
        assert_eq!(result, "2-5G-3J");
    }

    #[test]
    fn example_3() {
        let s = str!("a-a-a-a-");
        let k = 1;
        let result = Solution::license_key_formatting(s, k);
        assert_eq!(result, "A-A-A-A");
    }


}
