/// Given an array of integers `citations` where `citations[i]` is the number
/// of citations a researcher received for their `ith` paper, return the
/// researcher's h-index.
///
/// According to the definition of h-index on Wikipedia: The h-index is defined
/// as the maximum value of `h` such that the given researcher has published
/// at least `h` papers that have each been cited at least `h` times.
struct Solution;

impl Solution {

    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut times = vec![0; n+1];

        for cite in citations {
            let value = (cite as usize).min(n);
            times[value] += 1;
        }

        let mut count = 0;
        let mut index = n;
        let mut result = 0;

        while index > 0 {
            let value = times[index];
            count += value;
            if count >= index {
                result = index as i32;
                break;
            }
            index -= 1;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let citations = vec![3,0,6,1,5];
        let result = Solution::h_index(citations);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let citations = vec![1,3,1];
        let result = Solution::h_index(citations);
        assert_eq!(result, 1);
    }

    #[test]
    fn real_world_1() {
        let citations = vec![4,4,0,0];
        let result = Solution::h_index(citations);
        assert_eq!(result, 2);
    }

}
