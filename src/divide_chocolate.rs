/// You have one chocolate bar that consists of some chunks. Each chunk has its own sweetness given
/// by the array `sweetness`. 
///
/// You want to share the chocolate with your `k` friends so you start cutting the chocolate bar
/// into `k + 1` pieces using `k` cuts, each piece consists of some consecutive chunks.
///
/// Being generous, you will eat the piece with the minimum total sweetness and give the other
/// pieces to your friends.
///
/// Find the maximum total sweetness of the piece you can get by cutting the chocolate bar
/// optimally.
struct Solution;

impl Solution {

    fn minimum(sweetness: &Vec<i32>) -> i32 {
        let mut result = i32::MAX;
        for value in sweetness {
            result = result.min(*value);
        }
        result
    }

    fn maximum(sweetness: &Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        for value in sweetness {
            result += *value;
        }
        result / (k + 1)
    }

    fn is_valid_cut(sweetness: &Vec<i32>, k: i32, minimum: i32) -> bool {
        let mut current = 0;
        let mut groups = 0;

        for value in sweetness {
            current += *value;
            if current >= minimum {
                groups += 1;
                current = 0;
            }
        }

        groups >= k + 1
    }

    pub fn maximize_sweetness(sweetness: Vec<i32>, k: i32) -> i32 {
        let mut left = Self::minimum(&sweetness);
        let mut right = Self::maximum(&sweetness, k);

        while left <= right {
            let mid = left + (right - left) / 2;
            if Self::is_valid_cut(&sweetness, k, mid) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        left - 1
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let sweetness = vec![1,2,3,4,5,6,7,8,9];
        let k = 5;
        let result = Solution::maximize_sweetness(sweetness, k);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let sweetness = vec![5,6,7,8,9,1,2,3,4];
        let k = 8;
        let result = Solution::maximize_sweetness(sweetness, k);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let sweetness = vec![1,2,2,1,2,2,1,2,2];
        let k = 2;
        let result = Solution::maximize_sweetness(sweetness, k);
        assert_eq!(result, 5);
    }

}
