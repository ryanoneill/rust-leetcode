/// Write a function that takes the binary representation of an unsigned
/// integer and returns the number of `1` bits it has (also known as the
/// Hamming weight).
struct Solution;

impl Solution {

    // Should be hamming_weight, but the problem uses
    // hammingWeight
    #[allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        let mut result = 0;
        for byte in n.to_ne_bytes() {
            result += byte.count_ones();
        }
        result as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    fn example_1() {
        let n = 0b00000000000000000000000000001011;
        let result = Solution::hammingWeight(n);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let n = 0b00000000000000000000000010000000;
        let result = Solution::hammingWeight(n);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let n = 0b11111111111111111111111111111101;
        let result = Solution::hammingWeight(n);
        assert_eq!(result, 31);
    }

}
