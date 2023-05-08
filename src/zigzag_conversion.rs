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

    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 { s }
        else {
            let n = num_rows as usize;
            let mut down = true;
            let mut row = 0;
            let mut holding = vec![vec![]; n];

            for c in s.chars() {
                holding[row].push(c);
                if down {
                    if row == n-1 {
                        down = false;
                        row = n-2;
                    } else {
                        row += 1;
                    }
                } else {
                    if row == 0 {
                        down = true;
                        row = 1;
                    } else {
                        row -= 1;
                    }
                }
            }

            holding
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect()
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, "PAHNAPLSIIGYIR");
    }

    #[test]
    fn example_2() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 4;
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, "PINALSIGYAHRPI");
    }

    #[test]
    fn example_3() {
        let s = "A".to_string();
        let num_rows = 1;
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, "A");
    }

    #[test]
    fn two_rows() {
        let s = "ABCDEFGHIJKLMNOP".to_string();
        let num_rows = 2;
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, "ACEGIKMOBDFHJLNP");
    }

}
