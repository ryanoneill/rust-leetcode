/// You are playing a Flip Game with your friend.
///
/// You are given a string `currentState` that contains only `'+'` and `'-'`. You and your friend
/// take turns to flip two consecutive `"++"` into `"--"`. The game ends when a person can no
/// longer make a move, and therefore the other person will be the winner.
///
/// Return all possible states of the string `currentState` after one valid move. You may return
/// the answer in any order. If there is no valid move, return an empty list `[]`.
struct Solution;

impl Solution {

    pub fn generate_possible_next_moves(current_state: String) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();

        let values: Vec<char> = current_state.chars().collect();
        let n = values.len();

        let mut last = values[0];
        for i in 1..n {
            let value = values[i];
            match value {
                '+' => if last == '+' {
                    let mut result = values.clone();
                    result[i] = '-';
                    result[i-1] = '-';
                    results.push(result.into_iter().collect());
                }
                _ => { }
            }
            last = value;
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let current_state = "++++".to_string();
        let mut results = Solution::generate_possible_next_moves(current_state);
        results.sort_unstable();
        assert_eq!(results, vec!["++--", "+--+", "--++"]);
    }

    #[test]
    fn example_2() {
        let current_state = "+".to_string();
        let results = Solution::generate_possible_next_moves(current_state);
        assert_eq!(results, Vec::<String>::new());
    }

}

// 1. Write down the problem ✓
// 2. Clarify the scope ✓
// ** currentState.length >= 1 and currentState.length <= 500
// ** currentState[i] is either '+' or '-'.
// ** input: String
// ** output: Vec<String>
// 3. Write down test cases
// ** input: '+' output: []
// ** input: '++' output: ['--']
// ** input: '++-' output: ['---']
// ** input: '++++' output: ['--++', '++--', '-++-']
// 4. Describe and Write Down the Algorithm
// ** Convert the String into a Vec of chars.
// ** Iterate through the Vec of chars while keep track of the last character too.
// ** If this characters is a '+' and the last character was a '+', then clone the Vec of chars
//    and modify this and the last character to '-' and add it to the results Vec.
