use crate::number_additions::NumberAdditions;

/// A confusing number is a number that when rotated `180` degrees becomes a
/// different number with each digit valid.
///
/// We can rotate digits of a number by `180` degrees to form new digits.
///
/// * When `0`, `1`, `6`, `8`, and `9` are rotated `180` degrees, they become
///   `0`, `1`, `9`, `8`, and `6` respectively.
///
/// * When `2`, `3`, `4`, `5`, and `7` are rotated `180` degrees, they become
///   invalid.
///
/// Note that after rotating a number, we can ignore leading zeros.
///
/// * For example, after rotating `8000`, we have `0008` which is considered as
///   just `8`.
///
/// Given an integer `n`, return `true` if it is a confusing number, or `false`
/// otherwise.
struct Solution;

impl Solution {

    pub fn confusing_number(n: i32) -> bool {
        let mut valid = true;

        let n = n as u32;
        let digits = n.to_vec();
        let rotated_digits = digits.into_iter().rev().map(|d| {
            match d {
                0 => 0,
                1 => 1,
                2 => { valid = false; 2 }
                3 => { valid = false; 3 }
                4 => { valid = false; 4 }
                5 => { valid = false; 5 }
                6 => 9, 
                7 => { valid = false; 7 }
                8 => 8,
                9 => 6,
                _ => { panic!("Unexpected") }
            }
        }).collect();
        let rotated: u32 = NumberAdditions::from_vec(rotated_digits);
        valid && (rotated != n)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 6;
        let result = Solution::confusing_number(n);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let n = 89;
        let result = Solution::confusing_number(n);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let n = 1111;
        let result = Solution::confusing_number(n);
        assert!(!result);
    }

}
