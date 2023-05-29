/// Design a parking system for a parking lot. The parking lot has three
/// kinds of parking spaces: big, medium, and small, with a fixed number of
/// slots for each size.
///
/// Implement the `ParkingSystem` class:
///
/// * `ParkingSystem(int big, int medium, int small)` Initializes object of the
///   `ParkingSystem` class. The number of slots for each parking space are
///   given as part of the constructor.
///
/// * `bool addCar(int carType)` Checks whether there is a parking space of
///   `carType` for the car that wants to get into the parking lot. `carType`
///   can be of three kinds: big, medium, or small, which are represented by
///   `1`, `2`, and `3` respectively. A car can only park in a parking space
///   of its `carType`. If there is no space available, return `false`, else
///   park the car in that size space and return `true`.
///
struct ParkingSystem {
    big_remaining: usize,
    medium_remaining: usize,
    small_remaining: usize,
}

impl ParkingSystem {

    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            big_remaining: big as usize,
            medium_remaining: medium as usize,
            small_remaining: small as usize
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            1 => self.add_big_car(),
            2 => self.add_medium_car(),
            3 => self.add_small_car(),
            _ => panic!("Unknown car size")
        }
    }

    fn add_big_car(&mut self) -> bool {
        let result = self.big_remaining > 0;
        if result {
            self.big_remaining -= 1;
        }
        result
    }

    fn add_medium_car(&mut self) -> bool {
        let result = self.medium_remaining > 0;
        if result {
            self.medium_remaining -= 1;
        }
        result
    }

    fn add_small_car(&mut self) -> bool {
        let result = self.small_remaining > 0;
        if result {
            self.small_remaining -= 1;
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::ParkingSystem;

    #[test]
    fn example_1() {
        let mut parking_system = ParkingSystem::new(1,1,0);
        let result = parking_system.add_car(1);
        assert!(result);
        let result = parking_system.add_car(2);
        assert!(result);
        let result = parking_system.add_car(3);
        assert!(!result);
        let result = parking_system.add_car(1);
        assert!(!result);
    }

}
