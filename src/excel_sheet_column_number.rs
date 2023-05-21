/// Given a string `columnTitle` that represents the column title as appears
/// in an Excel sheet, return its corresponding column number.
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

    pub fn title_to_number(column_title: String) -> i32 {
        let base = 26 as i32;
        let n = column_title.len();
        let mut result = 0;
        let mut i = 0;

        for letter in column_title.chars() {
            let ordinal = (letter as u32 - 64) as i32;
            let power = (n - 1 - i) as u32;
            result += ordinal * base.pow(power);
            i += 1;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    fn example_1() {
        let column_title = "A".to_string();
        let result = Solution::title_to_number(column_title);
        assert_eq!(result, 1);
    }

    // #[test]
    // fn example_2() {
    //     let column_title = "AB".to_string();
    //     let result = Solution::title_to_number(column_title);
    //     assert_eq!(result, 2);
    // }

    // #[test]
    // fn example_3() {
    //     let column_title = "ZY".to_string();
    //     let result = Solution::title_to_number(column_title);
    //     assert_eq!(result, 701);
    // }

    #[test]
    fn maximum() {
        let column_title = "FXSHRXW".to_string();
        let result = Solution::title_to_number(column_title);
        assert_eq!(result, i32::MAX);
    }

}
