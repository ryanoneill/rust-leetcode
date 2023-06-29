/// You are a product manager and currently leading a team to develop a new product. Unfortunately,
/// the latest version of your product fails the quality check. Since each version is developed
/// based on the previous version, all the versions after a bad version are also bad.
///
/// Suppose you have `n` versions `[1, 2, ..., n]` and you want to find out the first bad one,
/// which causes all the following ones to be bad.
///
/// You are given an API `bool isBadVersion(version)` which returns whether `version` is bad.
/// Implement a function to find the first bad version. You should minimize the number of calls to
/// the API.
struct Solution {
    bad: i32,
}

impl Solution {

    // This method naming convention is provided by LeetCode
    #[allow(non_snake_case)]
    fn isBadVersion(&self, n: i32) -> bool {
        n >= self.bad
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 2;
        let mut right = n;
        let result;

        if self.isBadVersion(1) {
            result = 1;
        } else {
            loop {
                let mid = left + (right - left) / 2;
                if self.isBadVersion(mid) {
                    if self.isBadVersion(mid - 1) {
                        right = mid - 2;
                    } else {
                        result = mid;
                        break;
                    }
                } else {
                    left = mid + 1;
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
        let n = 5;
        let bad = 4;
        let solution = Solution { bad };
        let result = solution.first_bad_version(n);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let n = 1;
        let bad = 1;
        let solution = Solution { bad };
        let result = solution.first_bad_version(n);
        assert_eq!(result, 1);
    }

}

