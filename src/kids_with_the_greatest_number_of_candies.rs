/// There are `n` kids with candies. You are given an integer array `candies`,
/// where each `candies[i]` represents the number of candies the `ith` kid has,
/// and an integer `extraCandies`, denoting the number of extra candies that
/// you have.
///
/// Return a boolean array `result` of length `n`, where `result[i]` is `true`
/// if, after giving the `ith` kid all the `extraCandies`, they will have the
/// greatest number of candies among all the kids, or `false` otherwise.
///
/// Note that multiple kids can have the greatest number of candies.
struct Solution;

impl Solution {

    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_num = candies.iter().max().copied();
        max_num.map(|max| {
            candies
                .iter()
                .map(|c| (*c + extra_candies) >= max)
                .collect()
        }).unwrap_or(Vec::new())
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let candies = vec![2,3,5,1,3];
        let extra_candies = 3;
        let result = Solution::kids_with_candies(candies, extra_candies);
        assert_eq!(result, vec![true, true, true, false, true]);
    }


    #[test]
    fn example_2() {
        let candies = vec![4,2,1,1,2];
        let extra_candies = 1;
        let result = Solution::kids_with_candies(candies, extra_candies);
        assert_eq!(result, vec![true, false, false, false, false]);
    }

    #[test]
    fn example_3() {
        let candies = vec![12, 1, 12];
        let extra_candies = 10;
        let result = Solution::kids_with_candies(candies, extra_candies);
        assert_eq!(result, vec![true, false, true]);
    }

}
