struct Solution;

impl Solution {

    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut result = "".to_string();

        let m = str1.len();
        let n = str2.len();

        let mut a = m;
        let mut b = n;

        while b > 0 {
            let t = b;
            b = a % b;
            a = t;
        }

        let pattern1 = &str1[0..a];
        let pattern2 = &str2[0..a];

        if pattern1 == pattern2 {
            let matches = (pattern1.repeat(m / a) == str1) &&
                (pattern1.repeat(n / a) == str2);
            if matches {
                result = pattern1.to_string();
            }
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let str1 = "ABCABC".to_string();
        let str2 = "ABC".to_string();
        let result = Solution::gcd_of_strings(str1, str2);
        assert_eq!(result, "ABC");
    }

    #[test]
    fn example_2() {
        let str1 = "ABABAB".to_string();
        let str2 = "ABAB".to_string();
        let result = Solution::gcd_of_strings(str1, str2);
        assert_eq!(result, "AB");
    }

    #[test]
    fn example_3() {
        let str1 = "LEET".to_string();
        let str2 = "CODE".to_string();
        let result = Solution::gcd_of_strings(str1, str2);
        assert_eq!(result, "");
    }

}
