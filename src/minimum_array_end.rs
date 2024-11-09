/// You are given two integers `n` and `x`. You have to construct an array of
/// positive integers `nums` of size `n` where for every `0 <= i < n - 1`, `nums[i + 1]`
/// is greater than `nums[i]`, and the result of the bitwise `AND` operation between all elements
/// of `nums` is `x`. 
///
/// Return the minimum possible value of `nums[n - 1]`.
struct Solution;

impl Solution {

    pub fn min_end(n: i32, x: i32) -> i64 {
        let x = x as i64;
        let mut result = x;

        for _ in 1..n {
            result = (result + 1) | x;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 3;
        let x = 4;
        let result = Solution::min_end(n, x);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let n = 2;
        let x = 7;
        let result = Solution::min_end(n, x);
        assert_eq!(result, 15);
    }

}
