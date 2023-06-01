use std::collections::HashMap;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct CheckIn {
    id: i32,
    station_name: String,
    time: i32,
}

impl CheckIn {
    
    fn new(id: i32, station_name: String, time: i32) -> Self {
        Self { id, station_name, time }
    }

}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Trips {
    count: usize,
    total_time: i64,
}

impl Trips {

    fn new(time: i32) -> Self {
        Self { count: 1, total_time: time as i64 }
    }

    fn add(&mut self, time: i32) {
        self.count += 1;
        self.total_time += time as i64;
    }

    fn get_average_time(&self) -> f64 {
        self.total_time as f64 / self.count as f64
    }

}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Leg {
    start: String,
    end: String,
}

impl Leg {

    fn new(start: String, end: String) -> Self {
        Self { start, end }
    }

}

/// An underground railway system is keeping track of customer travel times
/// between different stations. They are using this data to calculate the
/// average time it takes to travel from one station to another.
///
/// Implement the `UndergroundSystem` class:
///
/// * `void checkIn(int id, string stationName, int t)`
///   * A customer with a card ID equal to `id`, checks in at the station
///     `stationName` at time `t`.
///
///   * A customer can only be checked into one place at a time.
///
/// * `void checkOut(int id, string stationName, int t)`
///   * A customer with card ID equal to `id`, checks out from the station
///     `stationName` at time `t`.
///
/// * `double getAverageTime(string startStation, string endStation)`
///   * Returns the average time it takes to travel from `startStation` to
///     `endStation`.
///
///   * The average time is computed from all the previous traveling times from
///     `startStation` to `endStation` that happened directly, meaning a check
///     in at `startStation` followed by a check out from `endStation`.
///
///   * The time it takes to travel from `startStation` to `endStation` may be
///     different from the time it takes to travel from `endStation` to
///     `startStation`.
///
///   * There will be at least one customer that has traveled from
///     `startStation` to `endStation` before `getAverageTime` is called.
///
/// You may assume all calls to the `checkIn` and `checkOut` methods are
/// consistent. If a customer checks in a time `t1` and checks out at time
/// `t2`, then `t1 < t2`. All events happen in chronological order.
struct UndergroundSystem {
    check_ins: HashMap<i32, CheckIn>,
    times: HashMap<Leg, Trips>
}

impl UndergroundSystem {

    fn new() -> Self {
        Self { check_ins: HashMap::new(), times: HashMap::new() }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        let check_in = CheckIn::new(id, station_name, t);
        self.check_ins.insert(id, check_in);
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let check_in = self.check_ins.remove(&id);
        match check_in {
            Some(ci) => {
                let time = t - ci.time;
                let leg = Leg::new(ci.station_name, station_name);
                self
                    .times
                    .entry(leg)
                    .and_modify(|trips| trips.add(time))
                    .or_insert(Trips::new(time));
            }
            None => { }
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let leg = Leg::new(start_station, end_station);
        let trips = self.times[&leg];
        trips.get_average_time()
    }

}

#[cfg(test)]
mod tests {
    use super::UndergroundSystem;

    #[test]
    fn example_1() {
        let mut underground_system = UndergroundSystem::new();
        underground_system.check_in(45, "Leyton".to_string(), 3);
        underground_system.check_in(32, "Paradise".to_string(), 8);
        underground_system.check_in(27, "Leyton".to_string(), 10);
        underground_system.check_out(45, "Waterloo".to_string(), 15);
        underground_system.check_out(27, "Waterloo".to_string(), 20);
        underground_system.check_out(32, "Cambridge".to_string(), 22);
        let result = underground_system.get_average_time("Paradise".to_string(), "Cambridge".to_string());
        assert_eq!(result, 14.0);
        let result = underground_system.get_average_time("Leyton".to_string(), "Waterloo".to_string());
        assert_eq!(result, 11.0);
        underground_system.check_in(10, "Leyton".to_string(), 24);
        let result = underground_system.get_average_time("Leyton".to_string(), "Waterloo".to_string());
        assert_eq!(result, 11.0);
        underground_system.check_out(10, "Waterloo".to_string(), 38);
        let result = underground_system.get_average_time("Leyton".to_string(), "Waterloo".to_string());
        assert_eq!(result, 12.0);
    }

    #[test]
    fn example_2() {
        let mut underground_system = UndergroundSystem::new();
        underground_system.check_in(10, "Leyton".to_string(), 3);
        underground_system.check_out(10, "Paradise".to_string(), 8); 
        let result = underground_system.get_average_time("Leyton".to_string(), "Paradise".to_string());
        assert_eq!(result, 5.0);
        underground_system.check_in(5, "Leyton".to_string(), 10);
        underground_system.check_out(5, "Paradise".to_string(), 16);
        let result = underground_system.get_average_time("Leyton".to_string(), "Paradise".to_string());
        assert_eq!(result, 5.5);
        underground_system.check_in(2, "Leyton".to_string(), 21);
        underground_system.check_out(2, "Paradise".to_string(), 30);
        let result = underground_system.get_average_time("Leyton".to_string(), "Paradise".to_string());
        assert_eq!(result, 20.0 / 3.0)
    }

}
