/// The Hamming distance between two integers is the number of positions at which the corresponding
/// bits are different.
///
/// Given two integers `x` and `y`, return the Hamming distance between them.
struct Solution;

impl Solution {

    fn byte_to_bits(byte: u8) -> Vec<bool> {
        let mut byte = byte;
        let mut result = vec![false; 8];
        if byte >= 128 {
            result[0] = true;
            byte -= 128;
        }
        if byte >= 64 {
            result[1] = true;
            byte -= 64;
        }
        if byte >= 32 {
            result[2] = true;
            byte -= 32;
        }
        if byte >= 16 {
            result[3] = true;
            byte -= 16;
        }
        if byte >= 8 {
            result[4] = true;
            byte -= 8;
        }
        if byte >= 4 {
            result[5] = true;
            byte -= 4;
        }
        if byte >= 2 {
            result[6] = true;
            byte -= 2;
        }
        if byte >= 1 {
            result[7] = true;
        }

        result
    }

    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let x_bytes = x.to_ne_bytes();
        let y_bytes = y.to_ne_bytes();

        let x_bits: Vec<bool> = x_bytes.iter().flat_map(|byte| Self::byte_to_bits(*byte)).collect();
        let y_bits: Vec<bool> = y_bytes.iter().flat_map(|byte| Self::byte_to_bits(*byte)).collect();

        let n = x_bits.len();
        let mut result = 0;
        for i in 0..n {
            if x_bits[i] != y_bits[i] {
                result += 1;
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
        let x = 1;
        let y = 4;
        let result = Solution::hamming_distance(x, y);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let x = 3;
        let y = 1;
        let result = Solution::hamming_distance(x, y);
        assert_eq!(result, 1);
    }

}
