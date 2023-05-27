struct Solution;

impl Solution {

    pub fn int_to_roman(num: i32) -> String {
        let mut result = Vec::new();
        let mut num = num;

        loop {
            if num >= 1000 {
                result.push('M');
                num -= 1000;
            } else if num >= 900 {
                result.push('C');
                result.push('M');
                num -= 900;
            } else if num >= 500 {
                result.push('D');
                num -= 500;
            } else if num >= 400 {
                result.push('C');
                result.push('D');
                num -= 400;
            } else if num >= 100 {
                result.push('C');
                num -= 100;
            } else if num >= 90 {
                result.push('X');
                result.push('C');
                num -= 90;
            } else if num >= 50 {
                result.push('L');
                num -= 50;
            } else if num >= 40 {
                result.push('X');
                result.push('L');
                num -= 40;
            } else if num >= 10 {
                result.push('X');
                num -= 10;
            } else if num >= 9 {
                result.push('I');
                result.push('X');
                num -= 9;
            } else if num >= 5 {
                result.push('V');
                num -= 5;
            } else if num >= 4 {
                result.push('I');
                result.push('V');
                num -= 4;
            } else if num >= 1 {
                result.push('I');
                num -= 1;
            } else {
                break;
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
        let num = 3;
        let result = Solution::int_to_roman(num);
        assert_eq!(result, "III");
    }

    #[test]
    fn example_2() {
        let num = 58;
        let result = Solution::int_to_roman(num);
        assert_eq!(result, "LVIII");
    }

    #[test]
    fn example_3() {
        let num = 1994;
        let result = Solution::int_to_roman(num);
        assert_eq!(result, "MCMXCIV");
    }

}
