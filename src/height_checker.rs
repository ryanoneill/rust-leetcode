/// A school is trying to take an annual photo of all the students. The
/// students are asked to stand in a single file line in non-decreasing order
/// by height. Let this ordering be represented by the integer array `expected`
/// where `expected[i]` is the expected height of the `ith` student in line.
///
/// You are given an integer array `heights` representing the current order
/// that the students are standing in. Each `heights[i]` is the height of the
/// `ith` student in line (0-indexed).
///
/// Return the number of indices where `heights[i] != expected[i]`.
struct Solution;

impl Solution {

    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut copy = heights.to_vec();
        copy.sort();

        heights
            .iter()
            .zip(copy.iter())
            .filter(|(&a, &b)| a != b)
            .count() as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let heights = vec![1, 1, 4, 2, 1, 3];
        let result = Solution::height_checker(heights);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let heights = vec![5,1,2,3,4];
        let result = Solution::height_checker(heights);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_3() {
        let heights = vec![1,2,3,4,5];
        let result = Solution::height_checker(heights);
        assert_eq!(result, 0);
    }

}
