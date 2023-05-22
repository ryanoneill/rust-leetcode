/// You are given a string `s` containing lowercase English letters, and a
/// matrix `shift`, where `shift[i] = [directioni, amounti]`:
///
/// * `directioni` can be `0` (for left shift) or `1` (for right shift).
///
/// * `amounti` is the amount by which string `s` is to be shifted.
///
/// * A left shift by 1 means remove the first character of `s` and append it
///   to the end.
///
/// * Similarly, a right shift by 1 means remove the last character of `s` and
///   add it to the beginning.
///
/// Return the final string after all operations.
struct Solution;

impl Solution {

    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let mut amount = shift
            .into_iter()
            .map(|sh| {
                let direction = sh[0];
                let amount = sh[1];
                if direction == 0 {
                    amount
                } else {
                    amount * -1
                }
            })
            .sum::<i32>();
        let n = s.len() as i32;
        if amount < 0 {
            amount = amount * -1;
            amount = amount % n;
            amount = n - amount;
        } else {
            amount = amount % n;
        }

        let (beginning, end) = s.split_at(amount as usize);
        end.to_string() + beginning
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "abc".to_string();
        let shift = vec![vec![0,1], vec![1,2]];
        let result = Solution::string_shift(s, shift);
        assert_eq!(result, "cab");
    }

    #[test]
    fn example_2() {
        let s = "abcdefg".to_string();
        let shift = vec![vec![1,1], vec![1,1], vec![0,2], vec![1,3]];
        let result = Solution::string_shift(s, shift);
        assert_eq!(result, "efgabcd");
    }

    #[test]
    fn no_shift() {
        let s = "abcdef".to_string();
        let shift = vec![];
        let result = Solution::string_shift(s, shift);
        assert_eq!(result, "abcdef");
    }

    #[test]
    fn real_world_1() {
        let s = "yzeuobhwxatulpruiab".to_string();
        let shift = vec![vec![1,15], vec![0,18], vec![0,12], vec![0,7], vec![0,7], vec![1,17],
            vec![1,15], vec![0,9], vec![1,4], vec![0,19], vec![1,16], vec![0,7],
            vec![1,4], vec![0,19], vec![1,16], vec![0,7], vec![1,4], vec![1,17],
            vec![1,19], vec![1,10], vec![1,2], vec![0,18], vec![1,15]];
        let result = Solution::string_shift(s, shift);
        assert_eq!(result, "wxatulpruiabyzeuobh");
    }

}
