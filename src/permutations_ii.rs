/// Given a collection of numbers, `nums`, that might contain duplicates, return all possible
/// unique permutations *in any order*.
struct Solution;

impl Solution {

    // Range is -10..10 for nums (21 numbers)
    fn to_index(num: i32) -> usize {
        (num + 10) as usize
    }

    fn from_index(index: usize) -> i32 {
        (index as i32) - 10 
    }

    fn to_freq(nums: &Vec<i32>) -> Vec<usize> {
        let mut results = vec![0; 21];
        for num in nums {
            let index = Self::to_index(*num);
            results[index] += 1;
        }
        results
    }

    fn add(freqs: &mut Vec<usize>, num: i32) {
        let index = Self::to_index(num);
        freqs[index] += 1;
    }

    fn remove(freqs: &mut Vec<usize>, num: i32) {
        let index = Self::to_index(num);
        if freqs[index] > 0 {
            freqs[index] -= 1;
        }
    }

    fn is_empty(freqs: &mut Vec<usize>) -> bool {
        let mut result = true;
        for i in 0..21 {
            if freqs[i] != 0 {
                result = false;
                break;
            }
        }
        result
    }

    fn worker(results: &mut Vec<Vec<i32>>, part: Vec<i32>, freqs: &mut Vec<usize>) {
        if Self::is_empty(freqs) {
            results.push(part);
        } else {
            for i in 0..21 {
                if freqs[i] > 0 {
                    let num = Self::from_index(i);
                    let mut attempt = part.clone();
                    attempt.push(num);
                    Self::remove(freqs, num);
                    Self::worker(results, attempt, freqs);
                    Self::add(freqs, num);
                }
            }
        }
    }

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut freqs = Self::to_freq(&nums);
        let mut results = Vec::new();
        let part = Vec::new();

        Self::worker(&mut results, part, &mut freqs);

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1, 1, 2];
        let mut result = Solution::permute_unique(nums);
        result.sort();
        assert_eq!(result, vec![
            vec![1, 1, 2],
            vec![1, 2, 1],
            vec![2, 1, 1],
        ]);
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 2, 3];
        let mut result = Solution::permute_unique(nums);
        result.sort();
        assert_eq!(result, vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]);
    }

}
