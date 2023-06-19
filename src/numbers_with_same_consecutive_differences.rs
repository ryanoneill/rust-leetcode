/// Given two integers n and k, return an array of all the integers of length `n` where the
/// difference between every two consecutive digits is `k`. You may return the answer in any order.
///
/// Note that the integers should not have leading zeros. Integers as `02` and `043` are not
/// allowed.
struct Solution;

impl Solution {

    fn backtrack(results: &mut Vec<i32>, n: usize, k: i32, current: Vec<i32>, num: i32) {
        if current.len() == n {
            let mut value = 0;
            for digit in current {
                value *= 10;
                value += digit;
            }
            results.push(value);
        } else {
            let added = num + k;

            if added >= 0 && added <= 9 {
                let mut current_added = current.clone();
                current_added.push(added);
                Self::backtrack(results, n, k, current_added, added);
            }

            if k > 0 {
                let subtracted = num - k;
                if subtracted >= 0 && subtracted <= 9 {
                    let mut current_subtracted = current.clone();
                    current_subtracted.push(subtracted);
                    Self::backtrack(results, n, k, current_subtracted, subtracted);
                }
            }
        }
    }

    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let n = n as usize;
        let mut results = Vec::new();

        // Number can't start with 0
        for i in 1..=9 {
            let num = i as i32;
            let current = vec![num];
            Self::backtrack(&mut results, n, k, current, num);
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 3;
        let k = 7;
        let mut result = Solution::nums_same_consec_diff(n, k);
        result.sort();
        assert_eq!(result, vec![181,292,707,818,929]);
    }

    #[test]
    fn exaple_2() {
        let n = 2;
        let k = 1;
        let mut result = Solution::nums_same_consec_diff(n, k);
        result.sort();
        assert_eq!(result, vec![10,12,21,23,32,34,43,45,54,56,65,67,76,78,87,89,98]);
    }

    #[test]
    fn k_zero() {
        let n = 2;
        let k = 0;
        let mut result = Solution::nums_same_consec_diff(n, k);
        result.sort();
        assert_eq!(result, vec![11,22,33,44,55,66,77,88,99]);
    }

}
