use std::collections::VecDeque;

/// In the world of Dota2, there are two parties: the Radiant and the Dire.
///
/// The Dota2 senate consists of senators coming from two parties. Now the
/// Senate wants to decide on a change in the Dota2 game. The voting for this
/// change is a round-based procedure. In each round, each senator can exercise
/// one of the two rights:
///
/// * Ban one senator's right: A senator can make another senator lose all his
///   rights in this and all the following rounds.
///
/// * Announce the victory: If this senator found the senators who still have
///   rights to vote are all from the same party, he can announce the victory
///   and decide on the change in the game.
///
/// Given a string `senate` representing each senator's party belonging. The
/// character `R` and `D` represent the Radiant party and the Dire party. Then
/// if there are `n` senators, the size of the given string will be `n`.
///
/// The round-based procedure starts from the first senator to the last senator
/// in the given order. This procedure will last until the end of voting. All
/// the senators who have lost their rights will be skipped during the
/// procedure.
///
/// Suppose every senator is smart enough and will play the best strategy for
/// his own party. Predict which party will finally announce the victory and
/// change the Dota2 game. The output should be `"Radiant"` or `"Dire"`.
struct Solution;

impl Solution {

    pub fn predict_party_victory(senate: String) -> String {
        let mut radiant_queue = VecDeque::new();
        let mut dire_queue = VecDeque::new();

        let n = senate.len();
        let mut i = 0;
        for c in senate.chars() {
            match c {
                'D' => { dire_queue.push_back(i); },
                'R' => { radiant_queue.push_back(i); },
                _   => { }
            }
            i += 1;
        }

        while !radiant_queue.is_empty() && !dire_queue.is_empty() {
            let radiant = radiant_queue.pop_front().unwrap();
            let dire = dire_queue.pop_front().unwrap();

            if dire < radiant {
                dire_queue.push_back(n + dire);
            } else {
                radiant_queue.push_back(n + radiant);
            }
        }

        if radiant_queue.is_empty() {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let senate = "RD".to_string();
        let result = Solution::predict_party_victory(senate);
        assert_eq!(result, "Radiant");
    }

    #[test]
    fn example_2() {
        let senate = "RDD".to_string();
        let result = Solution::predict_party_victory(senate);
        assert_eq!(result, "Dire");
    }

}
