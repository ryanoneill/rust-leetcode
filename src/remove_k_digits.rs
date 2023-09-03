struct Solution;

impl Solution {

    fn to_char(digit: i32) -> char {
        match digit {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7', 
            8 => '8',
            9 => '9',
            _ => '0',
        }
    }

    fn to_digit(letter: char) -> i32 {
        match letter {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _   => 0,
        }
    }

    pub fn remove_kdigits(num: String, k: i32) -> String {
        let k = k as usize;
        let mut left = k;
        let mut stack = Vec::new();

        let n = num.len();

        for c in num.chars() {
            let digit = Self::to_digit(c);

            while left > 0 && !stack.is_empty() {
                let value: i32 = stack.last().copied().unwrap();
                if digit < value {
                    stack.pop();
                    left -= 1;
                } else {
                    break;
                }
            }
            stack.push(digit);
        }
        
        let mut result = String::from("");
        let m = stack.len();
        let mut non_zero = false;

        for i in 0..n-k {
            if i < m {
                let digit = stack[i];
                if non_zero || digit != 0 {
                    non_zero = true;
                    result.push(Self::to_char(digit));
                }
            }
        }

        if result == "" {
            result.push('0');
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let num = "1432219".to_string();
        let k = 3;
        let result = Solution::remove_kdigits(num, k);
        assert_eq!(result, "1219");
    }

    #[test]
    fn example_2() {
        let num = "10200".to_string();
        let k = 1;
        let result = Solution::remove_kdigits(num, k);
        assert_eq!(result, "200");
    }

    #[test]
    fn example_3() {
        let num = "10".to_string();
        let k = 2;
        let result = Solution::remove_kdigits(num, k);
        assert_eq!(result, "0");
    }

    #[test]
    fn example_4() {
        let num = "12345678".to_string();
        let k = 4;
        let result = Solution::remove_kdigits(num, k);
        assert_eq!(result, "1234");
    }

    #[test]
    fn example_5() {
        let num = "1234567890".to_string();
        let k = 9;
        let result = Solution::remove_kdigits(num, k);
        assert_eq!(result, "0");
    }

}
