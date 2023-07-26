/// There are `n` gas stations along a circular route, where the amount of gas at the `ith` station
/// is `gas[i]`.
///
/// You have a car with an unlimited gas tank and it costs `cost[i]` of gas to travel from the
/// `ith` station to its next `(i + 1)th` station. You begin the journey with an empty tank at one
/// of the gas stations.
///
/// Given two integer arrays `gas` and `cost`, return the starting gas station's index if you can
/// travel around the circuit once in the clockwise direction, otherwise return `-1`. If there
/// exists a solution, it is guaranteed to be unique.
struct Solution;

impl Solution {

    fn to_diffs(gas: &Vec<i32>, cost: &Vec<i32>) -> Vec<i32> {
        let n = gas.len();
        let mut results = Vec::with_capacity(n);
        for i in 0..n {
            results.push(gas[i] - cost[i]);
        }
        results
    }

    fn diffs_sum(diffs: &Vec<i32>) -> i64 {
        let mut result = 0;
        let n = diffs.len();
        for i in 0..n {
            result += diffs[i] as i64;
        }
        result
    }

    fn can_complete_circuit_from(diffs: &Vec<i32>, start: usize) -> bool {
        let mut current = 0;
        let n = diffs.len();

        for i in start..n {
            current += diffs[i];
            if current < 0 {
                break;
            }
        }

        current >= 0
    }

    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let diffs = Self::to_diffs(&gas, &cost);
        let diffs_sum = Self::diffs_sum(&diffs);
        let mut result = -1;

        if diffs_sum >= 0 {
            for i in 0..n {
                if diffs[i] >= 0 {
                    if Self::can_complete_circuit_from(&diffs, i) {
                        result = i as i32;
                        break;
                    }
                }
            }
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let gas = vec![1,2,3,4,5];
        let cost = vec![3,4,5,1,2];
        let result = Solution::can_complete_circuit(gas, cost);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let gas = vec![2,3,4];
        let cost = vec![3,4,3];
        let result = Solution::can_complete_circuit(gas, cost);
        assert_eq!(result, -1);
    }

}
