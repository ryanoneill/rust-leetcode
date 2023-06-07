/// You are given a floating-point number `hour`, representing the amount of
/// time you have to reach the office. To commute to the office, you must take
/// `n` trains in sequential order. You are also given an integer array `dist`
/// of length `n`, where `dist[i]` describes the distance (in kilometers) of
/// the `ith` train ride.
///
/// Each train can only depart at an integer hour, so you may need to wait in
/// between each train ride.
///
/// * For example, if the `1st` train ride takes `1.5` hours, you must wait for
/// an additional `0.5` hours before you can depart on the `2nd` train ride at
/// the 2 hour mark.
///
/// Return the minimum positive integer speed (in kilometers per hour) that all
/// trains must travel at for you to reach the office on time, or `-1` if it is
/// impossible to be on time.
struct Solution;

impl Solution {
    
    fn calculate_time(dist: &Vec<i32>, speed: i32) -> f64 {
        let mut result: f64 = 0.0;
        let n = dist.len();
        for i in 0..n {
            let d = dist[i];
            let time = d as f64 / speed as f64;
            if i == n-1 {
                result += time;
            } else {
                result += time.ceil();
            }
        }

        result
    }

    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        if dist.len() as f64 > hour.ceil() { -1 }
        else {
            let mut start = 1;
            let mut end = 10000000;

            while start <= end {
                let mid = start + (end - start) / 2;
                let time = Self::calculate_time(&dist, mid);
                let can_finish = time <= hour;
                if can_finish {
                    end = mid - 1;
                } else {
                    start = mid + 1;
                }
            }

            start
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let dist = vec![1,3,2];
        let hour = 6.0;
        let result = Solution::min_speed_on_time(dist, hour);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let dist = vec![1,3,2];
        let hour = 2.7;
        let result = Solution::min_speed_on_time(dist, hour);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_3() {
        let dist = vec![1,3,2];
        let hour = 1.9;
        let result = Solution::min_speed_on_time(dist, hour);
        assert_eq!(result, -1);
    }

}
