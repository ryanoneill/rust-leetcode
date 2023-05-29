/// Given an integer `n`, return an array `ans` of length `n + 1` such that for
/// each `i` (`0 <= i <= n`), `ans[i]` is the number of `1`s in the binary
/// representation of `i`.
struct Solution;

impl Solution {

    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut result = vec![0; n + 1];
        for i in 0..=n {
            result[i] = i.count_ones() as i32;
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 2;
        let result = Solution::count_bits(n);
        assert_eq!(result, vec![0,1,1]);
    }

    #[test]
    fn example_2() {
        let n = 5;
        let result = Solution::count_bits(n);
        assert_eq!(result, vec![0,1,1,2,1,2]);
    }

}
