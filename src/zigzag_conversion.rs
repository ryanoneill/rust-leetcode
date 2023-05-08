/// The string "PAYPALISHIRING" is written in zigzag pattern on a given number
/// of rows like this:
///
/// P   A   H   N
/// A P L S I I G
/// Y   I   R
///
/// And then read line by line: "PAHNAPLSIIGYIR"
///
/// Write the code that will take a string and make this conversion given a
/// number of rows:
struct Solution;

impl Solution {

    // TODO: Implement
    pub fn convert(_s: String, _num_rows: i32) -> String {
        "".to_string()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, "PAHNAPLSIIGYIR");
    }

    #[ignore]
    #[test]
    fn example_2() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 4;
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, "PINALSIGYAHRPI");
    }

    #[ignore]
    #[test]
    fn example_3() {
        let s = "A".to_string();
        let num_rows = 1;
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, "A");
    }

}
