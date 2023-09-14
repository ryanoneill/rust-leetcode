/// You are given two lists of closed intervals, `firstList` and `secondList`, where
/// `firstList[i] = [starti, endi]` and `secondList[i] = [starti, endi]`. Each list of intervals is
/// pairwise disjoint and in sorted order.
///
/// Return the intersection of these two interval lists.
///
/// A closed interval `[a, b]` (with `a <= b`) denotes the set of real numbers `x` with
/// `a <= x <= b`. 
///
/// The intersection of two closed intervals is a set of real numbers that are either empty or
/// represented as a closed interval. For example, the intersection of `[1,3]` and `[2,4]` is
/// `[2,3]`.
struct Solution;

impl Solution {

    pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut results = vec![];

        let m = first_list.len();
        let n = second_list.len();

        let mut i = 0;
        let mut j = 0;

        while i < m && j < n {
            let first_start = first_list[i][0];
            let first_end = first_list[i][1];
            let second_start = second_list[j][0];
            let second_end = second_list[j][1];

            let interval_start = first_start.max(second_start);
            let interval_end = first_end.min(second_end);

            if interval_start <= interval_end {
                let result = vec![interval_start, interval_end];
                results.push(result);
            }
            if first_end == interval_end {
                i += 1;
            } else {
                j += 1;
            }
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let first = vec![vec![0,2], vec![5,10], vec![13,23], vec![24,25]];
        let second = vec![vec![1,5], vec![8,12], vec![15,24], vec![25,26]];
        let result = Solution::interval_intersection(first, second);
        assert_eq!(result, vec![vec![1,2], vec![5,5], vec![8,10], vec![15,23], vec![24, 24], vec![25,25]]);
    }

    #[test]
    fn example_2() {
        let first = vec![vec![1,3], vec![5,9]];
        let second = vec![];
        let result = Solution::interval_intersection(first, second);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

}
