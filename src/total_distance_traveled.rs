/// A truck has two fuel tanks. You are given two integers, `mainTank` representing the fuel
/// present in the main tank in liters and `additionalTank` representing the fuel present in the
/// additional tank in liters.
///
/// The truck has a mileage of `10` km per liter. Whenever `5` liters of fuel get used up in the
/// main tank, if the additional tank has at least `1` liters of fuel, `1` liters of fuel will be
/// transferred from the additional tank to the main tank.
///
/// Return the maximum distance which can be traveled.
///
/// Note: Injection from the additional tank is not continuous. It happens suddenly and immediately
/// for every 5 liters consumed.
struct Solution;

impl Solution {

    pub fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
        let mut main_tank = main_tank;
        let mut additional_tank = additional_tank;

        let mut result = 0;
        while main_tank > 0 {
            if main_tank >= 5 {
                main_tank -= 5;
                result += 50;
                if additional_tank > 0 {
                    main_tank += 1;
                    additional_tank -= 1;
                }
            } else {
                main_tank -= 1;
                result += 10;
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
        let main_tank = 5;
        let additional_tank = 10;
        let result = Solution::distance_traveled(main_tank, additional_tank);
        assert_eq!(result, 60);
    }

    #[test]
    fn example_2() {
        let main_tank = 1;
        let additional_tank = 2;
        let result = Solution::distance_traveled(main_tank, additional_tank);
        assert_eq!(result, 10);
    }

}
