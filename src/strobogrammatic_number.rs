/// Given a string `num` which represents an integer, return `true` if `num` is
/// a strobogrammatic number.
///
/// A strobogrammatic number is a number that looks the same when rotated `180`
/// degrees (looked at upside down).
struct Solution;

impl Solution {

    pub fn is_strobogrammatic(num: String) -> bool {
        let mut result = true;
        let upside_down: String = num
            .chars()
            .rev()
            .map(|digit| { 
                match digit {
                    '0' => '0',
                    '1' => '1',
                    '2' => { result = false; '2' },
                    '3' => { result = false; '3' },
                    '4' => { result = false; '4' },
                    '5' => { result = false; '5' },
                    '6' => '9',
                    '7' => { result = false; '7' },
                    '8' => '8',
                    '9' => '6',
                    _   => { result = false; digit }

                }
            })
            .collect();

        result && num == upside_down
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let num = "69".to_string();
        let result = Solution::is_strobogrammatic(num);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let num = "88".to_string();
        let result = Solution::is_strobogrammatic(num);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let num = "962".to_string();
        let result = Solution::is_strobogrammatic(num);
        assert!(!result);
    }

}
