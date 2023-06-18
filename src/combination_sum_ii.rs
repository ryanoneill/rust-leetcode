/// Given a collection of candidate numbers (`candidates`) and a target number (`target`), find all
/// unique combinations in `candidates` where the candidate numbers sum to `target`.
///
/// Each number in `candidates` may only be used once in the combination.
///
/// Note: The solution set must not contain duplicate combinations.
struct Solution;

impl Solution {

    fn to_counts(candidates: Vec<i32>) -> Vec<usize> {
        let mut counts: Vec<usize> = vec![0; 51];
        for num in candidates {
            let i = num as usize;
            counts[i] += 1;
        }
        counts
    }

    fn backtrack(results: &mut Vec<Vec<i32>>, counts: &Vec<usize>, target: i32, current: Vec<i32>, current_sum: i32, i: usize, count: usize) {
        if current_sum > target || i > 50 {
            // Do Nothing
        } else if current_sum == target {
            results.push(current);
        } else {
            let i_count = counts[i];
            if count < i_count {
                let mut cloned = current.clone();
                cloned.push(i as i32);
                let cloned_sum = current_sum + (i as i32);
                Self::backtrack(results, counts, target, cloned, cloned_sum, i, count + 1);
            }
            for j in i+1..=50 {
                let j_count = counts[j];
                if j_count > 0 {
                    let mut cloned = current.clone();
                    cloned.push(j as i32);
                    let cloned_sum = current_sum + (j as i32);
                    Self::backtrack(results, counts, target, cloned, cloned_sum, j, 1);
                }
            }
        }
    }

    // Numbers in candidates can only be from [1,50]
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results = Vec::new();

        let counts = Self::to_counts(candidates);
        let current = vec![];
        Self::backtrack(&mut results, &counts, target, current, 0, 1, 0);

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let candidates = vec![10,1,2,7,6,1,5];
        let target = 8;
        let mut result = Solution::combination_sum2(candidates, target);
        result.sort();
        assert_eq!(result, vec![vec![1,1,6], vec![1,2,5], vec![1,7], vec![2,6]]);
    }

    #[test]
    fn example_2() {
        let candidates = vec![2,5,2,1,2];
        let target = 5;
        let mut result = Solution::combination_sum2(candidates, target);
        result.sort();
        assert_eq!(result, vec![vec![1,2,2], vec![5]]);
    }

    #[test]
    fn counts() {
        let candidates = vec![1,2];
        let target = 4;
        let result = Solution::combination_sum2(candidates, target);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

}
