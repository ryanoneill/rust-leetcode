use std::collections::HashMap;

/// You have planned some train traveling one year in advance. The days of the year in which you
/// will travel are given as an integer array `days`. Each day is an integer from `1` to `365`.
///
/// Train tickets are sold in three different ways:
///
/// * a 1-day pass is sold for `costs[0]` dollars,
///
/// * a 7-day pass is sold for `costs[1]` dollars, and
///
/// * a 30-day pass is sold for `costs[2]` dollars.
///
/// The passes allow that many days of consecutive travel.
///
/// * For example, if we get a 7-day pass on day `2`, then we can travel for `7` days: `2`, `3`,
/// `4`, `5`, `6`, `7`, and `8`.
///
/// Return the minimum number of dollars you need to travel every day in the given list of days.
struct Solution;

impl Solution {

    fn next_index(days: &Vec<i32>, i: usize, length: i32) -> usize {
        let day = days[i];
        let next_day = day + length;
        
        let mut result = i + 1;
        while result < days.len() {
            let current = days[result];
            if current < next_day {
                result += 1;
            } else {
                break;
            }
        }

        result
    }

    fn worker(results: &mut HashMap<usize, i32>, days: &Vec<i32>, costs: &Vec<i32>, i: usize) -> i32 {
        if i == days.len() {
            0
        } else if results.contains_key(&i) {
            results[&i]
        } else {
            let one_day = costs[0] + Self::worker(results, days, costs, i + 1);
            let seven_day = costs[1] + Self::worker(results, days, costs, Self::next_index(days, i, 7));
            let thirty_day = costs[2] + Self::worker(results, days, costs, Self::next_index(days, i, 30));

            let result = one_day.min(seven_day).min(thirty_day);
            results.insert(i, result);

            result
        }
    }

    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut results = HashMap::new();
        Self::worker(&mut results, &days, &costs, 0)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let days = vec![1,4,6,7,8,20];
        let costs = vec![2,7,15];
        let result = Solution::mincost_tickets(days, costs);
        assert_eq!(result, 11);
    }

    #[test]
    fn example_2() {
        let days = vec![1,2,3,4,5,6,7,8,9,10,30,31];
        let costs = vec![2,7,15];
        let result = Solution::mincost_tickets(days, costs);
        assert_eq!(result, 17);
    }

}
