/// Given 3 positive numbers `a`, `b`, and `c`. Return the minimum flips required in some bits of
/// `a` and `b` to make (`a` OR `b` == `c`). (bitwise OR operation). Flip operation consists of
/// change any single bit 1 to 0 or change the bit 0 to 1 in their binary representation.
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

    fn min_flips_byte(a_byte: u8, b_byte: u8, c_byte: u8) -> i32 {
        let a_bits = Self::byte_to_bits(a_byte);
        let b_bits = Self::byte_to_bits(b_byte);
        let c_bits = Self::byte_to_bits(c_byte);

        let mut result = 0;
        for i in 0..8 {
            match (c_bits[i], a_bits[i], b_bits[i]) {
                (true, false, false) => result += 1,
                (false, false, true) => result += 1,
                (false, true, false) => result += 1,
                (false, true, true) => result += 2,
                _ => { }
            }
        }

        result
    }

    fn min_flips_bytes(a_bytes: [u8; 4], b_bytes: [u8; 4], c_bytes: [u8; 4]) -> i32 {
        let mut result = 0;
        for i in 0..4 {
            result += Self::min_flips_byte(a_bytes[i], b_bytes[i], c_bytes[i])
        }
        result
    }

    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let a_bytes = a.to_ne_bytes();
        let b_bytes = b.to_ne_bytes();
        let c_bytes = c.to_ne_bytes();
        Self::min_flips_bytes(a_bytes, b_bytes, c_bytes)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let a = 2;
        let b = 6;
        let c = 5;
        let result = Solution::min_flips(a, b, c);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let a = 4;
        let b = 2;
        let c = 7;
        let result = Solution::min_flips(a, b, c);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let a = 1;
        let b = 2;
        let c = 3;
        let result = Solution::min_flips(a, b, c);
        assert_eq!(result, 0);
    }

}
