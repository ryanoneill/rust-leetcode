/// Given an integer `num`, return a string representating its hexadecimal representation. For
/// negative integers, two's complement method is used.
///
/// All the letters in the answer string should be lowercase characters, and there should not be
/// any leading zeros in the answer except for the zero itself.
struct Solution;

impl Solution {

    fn digit_to_hex(num: u32) -> char {
        match num {
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
            10 => 'a',
            11 => 'b',
            12 => 'c',
            13 => 'd',
            14 => 'e',
            15 => 'f',
            _  => '_',
        }
    }


    // -2^31 <= num <= 2^31 - 1
    pub fn to_hex(num: i32) -> String {
        let mut rep = num as u32;

        let mut result: Vec<char> = vec![];

        if rep == 0 {
            result.push('0');
        } else {
            while rep > 0 {
                let left = rep / 16;
                let current = rep % 16;
                result.push(Self::digit_to_hex(current));
                rep = left;
            }
        }

        result.into_iter().rev().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let num = 26;
        let result = Solution::to_hex(num);
        assert_eq!(result, "1a");
    }

    #[test]
    fn example_2() {
        let num = -1;
        let result = Solution::to_hex(num);
        assert_eq!(result, "ffffffff");
    }

    #[test]
    fn example_3() {
        let num = 5;
        let result = Solution::to_hex(num);
        assert_eq!(result, "5");
    }

    #[test]
    fn example_4() {
        let num = 10;
        let result = Solution::to_hex(num);
        assert_eq!(result, "a");
    }

}
