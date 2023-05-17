/// Given an integer `columnNumber`, return its corresponding column title as
/// it appears in an Excel sheet.
///
/// For example:
///
/// A -> 1
/// B -> 2
/// C -> 3
/// ...
/// Z -> 26
/// AA -> 27
/// AB -> 28
/// ...
struct Solution; 

impl Solution {

    pub fn convert_to_title(column_number: i32) -> String {
        let mut working = column_number - 1;
        let mut letters: Vec<char> = Vec::new();

        while working >= 0 {
            let value = (65 + working % 26) as u32;
            let c = char::from_u32(value).unwrap();
            letters.push(c);
            working = working / 26 - 1;
        }

        letters.iter().rev().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let column_number = 1;
        let result = Solution::convert_to_title(column_number);
        assert_eq!(result, "A");
    }

    #[test]
    fn example_2() {
        let column_number = 28;
        let result = Solution::convert_to_title(column_number);
        assert_eq!(result, "AB");
    }

    #[test]
    fn example_3() {
        let column_number = 701;
        let result = Solution::convert_to_title(column_number);
        assert_eq!(result, "ZY");
    }

}
