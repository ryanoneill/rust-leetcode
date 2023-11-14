use std::collections::HashMap;

/// You are given a 0-indexed 2D integer array `questions` where
/// `questions[i] = [pointsi, brainpoweri]`.
///
/// The array describes the questions of an exame, where you have to process
/// the questions in order (i.e. starting from question `0`) and make a
/// decision whether to solve or skip each question. Solving question `i` will
/// earn you `pointsi` points but you will be unable to solve each of the next
/// `brainpoweri` questions. If you skip question `i`, you get to make the
/// decision on the next question.
///
/// * For example, given `questions = [[3,2], [4,3], [4,4], [2,5]]`:
///   * If question `0` is solved, you will earn `3` points but you will be
///     unable to solve questions `1` and `2`.
///   * If instead, question `0` is skipped and question `1` is solved, you
///     will earn `4` points but you will be unable to solve questions `2` and
///     `3`.
///
/// Return the maximum points you can earn for the exam.
struct Solution;

impl Solution {

    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut points = HashMap::new();
        Self::worker(&questions, &mut points)
    }

    fn worker(questions: &[Vec<i32>], points: &mut HashMap<usize, i64>) -> i64 {
        let n = questions.len();
        match n {
            0 => 0,
            1 => questions[0][0] as i64,
            _ => {
                if points.contains_key(&n) {
                    points[&n]
                } else {
                    let question = &questions[0];
                    let value = question[0] as i64;
                    let power = question[1] as usize;
                    let next = n.min(power + 1);
                    let with_question = value + Self::worker(&questions[next..], points);

                    let without_question = Self::worker(&questions[1..], points);
                    let result = with_question.max(without_question);
                    points.insert(n, result);

                    result
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let questions = vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]];
        let result = Solution::most_points(questions);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_2() {
        let questions = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]];
        let result = Solution::most_points(questions);
        assert_eq!(result, 7);
    }
}
