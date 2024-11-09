use std::collections::BinaryHeap;

/// You are given an integer array `score` of size `n`, where `score[i]` is the score of the `ith`
/// athlete in a competition. All the scores are guaranteed to be unique.
///
/// The athletes are placed based on their scores, where the `1st` place athlete has the highest
/// score, the `2nd` place athlete has the `2nd` highest score, and so on. The placement of each
/// athlete determines their rank:
///
/// * The `1st` place athlete's rank is `"Gold Medal"`.
///
/// * The `2nd` place athlete's rank is `"Silver Medal"`.
///
/// * The `3rd` place athlete's rank is `"Bronze Medal"`.
///
/// * For the `4th` place to the `nth` place athlete, their rank is their placement number (i.e.,
/// the `xth` place athlete's rank is `"x"`).
///
/// Return an array `answer` of size `n` where `answer[i]` is the rank of the `ith` athlete.
struct Solution;

impl Solution {

    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let n = score.len();
        let mut result: Vec<String> = vec![String::from(""); n];
        let mut heap = BinaryHeap::with_capacity(n);

        for i in 0..n {
            let value = score[i];
            heap.push((value, i));
        }

        let mut place = 0;
        while !heap.is_empty() {
            place += 1;
            let (_value, i) = heap.pop().unwrap();
            match place {
                1 => { result[i] = String::from("Gold Medal"); }
                2 => { result[i] = String::from("Silver Medal"); }
                3 => { result[i] = String::from("Bronze Medal"); }
                _ => { result[i] = place.to_string(); }
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
        let score = vec![5,4,3,2,1];
        let result = Solution::find_relative_ranks(score);
        assert_eq!(result, vec!["Gold Medal","Silver Medal","Bronze Medal","4","5"]);
    }

    #[test]
    fn example_2() {
        let score = vec![10,3,8,9,4];
        let result = Solution::find_relative_ranks(score);
        assert_eq!(result, vec!["Gold Medal","5","Bronze Medal","Silver Medal","4"]);
    }

}
