/// Given an array of integers `arr` and two integers `k` and `threshold`, return the number of
/// sub-arrays of size `k` and average greater than or equal to `threshold`.
struct Solution;

impl Solution {

    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let threshold_sum: i64 = (k as i64) * (threshold as i64);

        let mut result = 0;
        let mut current_sum: i64 = 0;

        let k = k as usize;
        let n = arr.len();

        if n >= k {
            for i in 0..k {
                current_sum += arr[i] as i64;
            }
            if current_sum >= threshold_sum {
                result += 1;
            }

            for i in k..n {
                let old = i - k;
                current_sum -= arr[old] as i64;
                current_sum += arr[i] as i64;
                if current_sum >= threshold_sum {
                    result += 1;
                }
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
        let arr = vec![2,2,2,2,5,5,5,8];
        let k = 3;
        let threshold = 4;
        let result = Solution::num_of_subarrays(arr, k, threshold);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let arr = vec![11,13,17,23,29,31,7,5,2,3];
        let k = 3;
        let threshold = 5;
        let result = Solution::num_of_subarrays(arr, k, threshold);
        assert_eq!(result, 6);
    }

}
