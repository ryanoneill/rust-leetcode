struct Solution;

impl Solution {

    fn merge(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
        let mut a_iter = a.into_iter();
        let mut b_iter = b.into_iter();

        let mut a_value = a_iter.next();
        let mut b_value = b_iter.next();

        let mut result = Vec::with_capacity(a.len() + b.len());

        while a_value.is_some() || b_value.is_some() {
            match (a_value, b_value) {
                (Some(a), Some(b)) => {
                    if a <= b {
                        result.push(*a);
                        a_value = a_iter.next();
                    } else {
                        result.push(*b);
                        b_value = b_iter.next();
                    }
                }
                (Some(a), None) => {
                    result.push(*a);
                    a_value = a_iter.next();
                }
                (None, Some(b)) => {
                    result.push(*b);
                    b_value = b_iter.next();
                }
                _ => { }
            }
        }

        result
    }

    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let i = nums.partition_point(|&num| num < 0);
        let (below, above) = nums.split_at(i);
        let below_squared = below.into_iter().map(|x| x * x).rev().collect();
        let above_squared = above.into_iter().map(|x| x * x).collect();
        Self::merge(&below_squared, &above_squared)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![-4, -1, 0, 3, 10];
        let result = Solution::sorted_squares(nums);
        assert_eq!(result, vec![0,1,9,16,100]);
    }

    #[test]
    fn example_2() {
        let nums = vec![-7,-3,2,3,11];
        let result = Solution::sorted_squares(nums);
        assert_eq!(result, vec![4,9,9,49,121]);
    }

}
