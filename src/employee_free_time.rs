use crate::interval::Interval;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// We are given a list `schedule` of employees, which represents the working time for each
/// employee.
///
/// Each employee has a list of non-overlapping `Intervals`, and these intervals are in sorted
/// order.
///
/// Return the list of finite intervals representing common, positive-length free time for all
/// employees, also in sorted order.
struct Solution;

impl Solution {

    pub fn employee_free_time(schedule: Vec<Vec<Interval>>) -> Vec<Interval> {
        let mut min_heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();

        for employee in schedule {
            for block in employee {
               let item = (block.start, block.end);
               min_heap.push(Reverse(item));
            }
        }

        let mut results = vec![];
        let mut start = -1;

        while !min_heap.is_empty() {
            let item = min_heap.pop().unwrap().0;
            let item_start = item.0;
            let item_end = item.1;

            if start == -1 {
                // intialization
                start = item_end;
            } else if start >= item_start {
                // covered, extend next good interval if necessary
                start = start.max(item_end);
            } else {
                // open space
                let end = item_start;
                let diff = end - start;
                if diff > 0 {
                    let interval = Interval::new(start, end);
                    results.push(interval);
                }
                start = item_end;
            }
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use crate::interval::Interval;
    use super::Solution;

    #[test]
    fn example_1() {
        let schedule = vec![
            vec![
                Interval::new(1, 2),
                Interval::new(5, 6),
            ],
            vec![
                Interval::new(1, 3),
                Interval::new(4, 10),
            ]
        ];
        let results = Solution::employee_free_time(schedule);
        assert_eq!(results.len(), 1);
        assert_eq!(results, vec![Interval::new(3,4)]);
    }

    #[test]
    fn example_2() {
        let schedule = vec![
            vec![
                Interval::new(1,3),
                Interval::new(6,7),
            ],
            vec![
                Interval::new(2,4),
            ],
            vec![
                Interval::new(2,5),
                Interval::new(9,12),
            ]
        ];
        let results = Solution::employee_free_time(schedule);
        assert_eq!(results.len(), 2);
        assert_eq!(results, vec![Interval::new(5,6), Interval::new(7,9)]);
    }

    #[test]
    fn example_3() {
        let schedule = vec![
            vec![
                Interval::new(45,56),
                Interval::new(89,96),
            ],
            vec![
                Interval::new(5,21),
                Interval::new(57,74),
            ],
        ];
        let results = Solution::employee_free_time(schedule);
        assert_eq!(results.len(), 3);
        assert_eq!(results, vec![
            Interval::new(21,45),
            Interval::new(56,57),
            Interval::new(74,89),
        ]);
    }

}
