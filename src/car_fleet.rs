use std::cmp::Reverse;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Car {
    position: i32,
    speed: i32
}

impl Car {

    pub fn new(position: i32, speed: i32) -> Self {
        Self { position, speed }
    }

}

/// There are `n` cars going to the same destination along a one-lane road. The destination is
/// `target` miles away.
///
/// You are given two integer arrays `position` and `speed`, both of length `n`, where
/// `position[i]` is the position of the `ith` car and `speed[i]` is the speed of the `ith` car (in
/// miles per hour).
///
/// A car can never pass another car ahead of it, but it can catch up to it and drive bumper to
/// bumper at the same speed. The faster car will slow down to match the slower car's speed. The
/// distance between these two cars is ignored (i.e., they are assumed to have the same position).
///
/// A car fleet is some non-empty set of cars driving at the same position and same speed. Note
/// that a single car is also a car fleet.
///
/// If a car catches up to a car fleet right at the destination point, it will still be considered
/// as one car fleet.
///
/// Return the number of car fleets that will arrive at the destination.
struct Solution;

impl Solution {

    fn to_cars(position: Vec<i32>, speed: Vec<i32>) -> Vec<Car> {
        let n = position.len();
        let mut results = Vec::with_capacity(n);

        for i in 0..n {
            let pos = position[i];
            let sp  = speed[i];
            let car = Car::new(pos, sp);
            results.push(car);
        }

        results
    }

    fn can_catch_up(target: i32, car1: &Car, car2: &Car) -> bool {
        let mut result = false;
        if car2.speed > car1.speed {
            let p = (car2.position - car1.position) as f64;
            let s = (car1.speed - car2.speed) as f64;
            let t = p / s;
            let meet = (car2.position as f64) + (car2.speed as f64 * t);
            result = meet <= target as f64;
        }
        result
    }

    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars = Self::to_cars(position, speed);
        cars.sort_by_key(|c| Reverse(c.position));
        println!("{:?}", cars);

        let mut stack = Vec::new();
        for car in cars {
            match stack.len() {
                0 => {
                    println!("Pushing Car: {:?}", car);
                    stack.push(car);
                }
                n => {
                    let previous = stack[n-1];
                    if !Self::can_catch_up(target, &previous, &car) {
                        stack.push(car);

                    }
                }
            }
        }


        stack.len() as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let target = 12;
        let position = vec![10,8,0,5,3];
        let speed = vec![2,4,1,1,3];
        let result = Solution::car_fleet(target, position, speed);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let target = 10;
        let position = vec![3];
        let speed = vec![3];
        let result = Solution::car_fleet(target, position, speed);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let target = 100;
        let position = vec![0,2,4];
        let speed = vec![4,2,1];
        let result = Solution::car_fleet(target, position, speed);
        assert_eq!(result, 1);
    }

    #[test]
    fn real_world_1() {
        let target = 16;
        let position = vec![11,14,13,6];
        let speed = vec![2,2,6,7];
        let result = Solution::car_fleet(target, position, speed);
        assert_eq!(result, 2);
    }

    // Car1: P(14) S(2); 16
    // Car2: P(13) S(6); 19 *
    // Car3: P(11) S(2); 13
    // Car4: P(6)  S(7); 13

}
