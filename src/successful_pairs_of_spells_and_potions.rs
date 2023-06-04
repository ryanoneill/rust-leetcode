/// You are also given an integer success. A spell and potion pair is
/// considered successful if the product of their strengths is at least success.
///
/// Return an integer array pairs of length n where pairs[i] is the number of
/// potions that will form a successful pair with the ith spell.
struct Solution;

impl Solution {

    fn search(potions: &Vec<i32>, target: i32) -> usize {
        let mut start = 0;
        let mut end = potions.len() - 1;

        while start <= end {
            let mid = start + (end - start) / 2;

            if potions[mid] < target {
                start = mid + 1;
            } else if mid == 0 {
                break;
            } else {
                end = mid - 1;
            }
        }

        start
    }

    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let m = potions.len();
        let mut potions = potions;
        potions.sort_unstable();

        let n = spells.len();
        let mut result = vec![0; n];
        for i in 0..n {
            let spell = spells[i];
            // Spell cannot be 0
            let target = (success as f64 / spell as f64).ceil() as i32;
            let point = Self::search(&potions, target);
            result[i] = (m - point) as i32;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let spells = vec![5,1,3];
        let potions = vec![1,2,3,4,5];
        let success = 7;
        let result = Solution::successful_pairs(spells, potions, success);
        assert_eq!(result, vec![4,0,3]);
    }

    #[test]
    fn example_2() {
        let spells = vec![3,1,2];
        let potions = vec![8,5,8];
        let success = 16;
        let result = Solution::successful_pairs(spells, potions, success);
        assert_eq!(result, vec![2,0,2]);
    }

    #[test]
    fn real_world_1() {
        let spells = vec![15,8,19];
        let potions = vec![38,36,23];
        let success = 328;
        let result = Solution::successful_pairs(spells, potions, success);
        assert_eq!(result, vec![3,0,3]);
    }

}
