/// You have some apples and a basket that can carry up to `5000` units of weight.
///
/// Given an integer array `weight` where `weight[i]` is the weight of the `ith` apple, return the
/// maximum number of apples you can put in the basket.
struct Solution;

impl Solution {

    pub fn max_number_of_apples(weight: Vec<i32>) -> i32 {
        let n = weight.len();
        let mut result = 0;

        let mut weight = weight;
        weight.sort_unstable();
        let mut sum = 0;

        for i in 0..n {
            sum += weight[i];
            if sum <= 5000 {
                result += 1;
            } else {
                break;
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
        let weight = vec![100,200,150,1000];
        let result = Solution::max_number_of_apples(weight);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let weight = vec![900,950,800,1000,700,800];
        let result = Solution::max_number_of_apples(weight);
        assert_eq!(result, 5);
    }

}
