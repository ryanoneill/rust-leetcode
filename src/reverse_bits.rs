/// Reverse bits of a given 32 bits unsigned integer.
struct Solution;

impl Solution {

    pub fn reverse_bits(x: u32) -> u32 {
        let mut bytes = x.to_ne_bytes();
        // Reverse the bits in each byte
        for i in 0..bytes.len() {
            bytes[i] = bytes[i].reverse_bits();
        }
        // Then reverse the order of the bytes
        for i in 0..2 {
            bytes.swap(i, 4-1-i);
        }
        u32::from_ne_bytes(bytes)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let x = 43261596;
        let result = Solution::reverse_bits(x);
        assert_eq!(result, 964176192);
    }

    #[test]
    fn example_2() {
        let x = 4294967293;
        let result = Solution::reverse_bits(x);
        assert_eq!(result, 3221225471);
    }

}
