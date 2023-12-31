/// There is a car with `capacity` empty seats. The vehicle only drives east (i.e., it cannot turn
/// around and drive west).
///
/// You are given the integer `capacity` and an array `trips` where `trips[i] = [numPassengersi,
/// fromi, toi]` indicates that the `ith` trip has `numPassengersi` passengers and the locations to
/// pick them up and drop them off are `fromi` and `toi` respectively. The locations are given as
/// the number of kilometers due east from the car's initial location.
///
/// Return `true` if it is possible to pick up and rop off all passengers for all the given trips,
/// or `false` otherwise.
struct Solution;

impl Solution {

    fn farthest(trips: &Vec<Vec<i32>>) -> i32 {
        let mut result = i32::MIN;
        for trip in trips {
            result = result.max(trip[2]);
        }
        result
    }

    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut result = true;

        let farthest = Self::farthest(&trips) as usize;
        let n = farthest + 1;

        let mut passengers = vec![0; n];

        for trip in trips {
            let count = trip[0];
            let from = trip[1] as usize;
            let to = trip[2] as usize;

            passengers[from] += count;
            passengers[to] -= count;
        }

        let mut current = 0;
        for i in 0..n {
            current += passengers[i];
            if current > capacity {
                result = false;
                break;
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
        let trips = vec![
            vec![2, 1, 5],
            vec![3, 3, 7],
        ];
        let capacity = 4;
        let result = Solution::car_pooling(trips, capacity);
        assert!(!result);
    }

    #[test]
    fn example_2() {
        let trips = vec![
            vec![2, 1, 5],
            vec![3, 3, 7],
        ];
        let capacity = 5;
        let result = Solution::car_pooling(trips, capacity);
        assert!(result);
    }

}
