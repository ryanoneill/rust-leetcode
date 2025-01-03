#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>)
}

/// You are given a nested list of integers `nestedList`. Each element is either an integer or a
/// list whose elements may also be integers or other lists.
///
/// The depth of an integer is the number of lists that it is inside of. For example, the nested
/// list `[1,[2,2],[[3],2],1]` has each integer's value set to its depth.
///
/// Return the sum of each integer `nestedList` multiplied by its depth.
struct Solution;

impl Solution {

    fn worker(list: &Vec<NestedInteger>, level: i32) -> i32 {
        let n = list.len();

        let mut direct = 0;
        let mut nested = 0;

        for i in 0..n {
            match &list[i] {
                NestedInteger::Int(value) => {
                    direct += value;
                }
                NestedInteger::List(items) => {
                    nested += Self::worker(&items, level + 1);
                }
            }
        }

        (direct * level) + nested
    }

    pub fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
        Self::worker(&nested_list, 1)
    }

}

#[cfg(test)]
mod tests {
    use super::NestedInteger;
    use super::Solution;

    #[test]
    fn example_1() {
        let nested_list = vec![
            NestedInteger::List(vec![
                NestedInteger::Int(1),
                NestedInteger::Int(1),
            ]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![
                NestedInteger::Int(1),
                NestedInteger::Int(1),
            ])
        ];
        let result = Solution::depth_sum(nested_list);
        assert_eq!(result, 10);
    }

    #[test]
    fn example_2() {
        let nested_list = vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(4),
                NestedInteger::List(vec![
                    NestedInteger::Int(6),
                ])
            ])
        ];
        let result = Solution::depth_sum(nested_list);
        assert_eq!(result, 27);
    }

    #[test]
    fn example_3() {
        let nested_list = vec![
            NestedInteger::Int(0),
        ];
        let result = Solution::depth_sum(nested_list);
        assert_eq!(result, 0);
    }


}
