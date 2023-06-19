use std::collections::BTreeSet;

/// You are given an integer array `matches` where `matches[i] = [winneri, loseri]` indicates that
/// the player `winneri` defeated player `loseri` in a match.
///
/// Return a list `answer` of size `2` where:
///
/// * `answer[0]` is a list of all players that have not lost any matches.
///
/// * `answer[1]` is a list of all players that have lost exactly one match.
///
/// The values in the two lists should be returned in increasing order.
///
/// Note:
///
/// * You should only consider the players that have played at least one match.
///
/// * The testcases will be generated such that no two matches will have the same outcome.
struct Solution;

impl Solution {

    // Note: Using a `BTreeSet` here instead of a `HashSet` and then sorting the resultant
    // `undefeated` and `single_losers` vecs is slower. However, I wanted to have experience
    // with using a `BTreeSet` and looking at its performance. At some point, I'll change this
    // back to using `HashSets` instead, or go back to the original method of counting losses.
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut zero_losses = BTreeSet::new();
        let mut one_loss = BTreeSet::new();
        let mut multiple_losses = BTreeSet::new();

        let mut result = Vec::with_capacity(2);
        for the_match in matches {
            let winner = the_match[0];
            let loser = the_match[1];

            if !multiple_losses.contains(&winner) && !one_loss.contains(&winner) {
                zero_losses.insert(winner);
            }

            if !multiple_losses.contains(&loser) {
                if zero_losses.contains(&loser) {
                    zero_losses.remove(&loser);
                    one_loss.insert(loser);
                } else if one_loss.contains(&loser) {
                    one_loss.remove(&loser);
                    multiple_losses.insert(loser);
                } else {
                    one_loss.insert(loser);
                }
            }
        }

        let undefeated: Vec<i32> = zero_losses.into_iter().collect();
        result.push(undefeated);

        let single_losers: Vec<i32> = one_loss.into_iter().collect();
        result.push(single_losers);

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let matches = vec![vec![1,3], vec![2,3], vec![3,6], vec![5,6], vec![5,7], vec![4,5], vec![4,8], vec![4,9], vec![10,4], vec![10,9]];
        let result = Solution::find_winners(matches);
        assert_eq!(result, vec![vec![1,2,10], vec![4,5,7,8]]);
    }

    #[test]
    fn example_2() {
        let matches = vec![vec![2,3], vec![1,3], vec![5,4], vec![6,4]];
        let result = Solution::find_winners(matches);
        assert_eq!(result, vec![vec![1,2,5,6], vec![]]);
    }

}
