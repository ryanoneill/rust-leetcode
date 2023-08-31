use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Time {
    hour: i32,
    minutes: i32,
}

impl Time {

    pub fn new(hour: i32, minutes: i32) -> Self {
        Self { hour, minutes }
    }

    pub fn parse(s: &str) -> Self {
        let colon_index = s.find(':').unwrap();
        let hour = s[0..colon_index].parse::<i32>().unwrap();
        let minutes = s[colon_index+1..].parse::<i32>().unwrap();
        Self::new(hour, minutes)
    }

    pub fn total(&self) -> i32 {
        self.hour * 60 + self.minutes
    }

    pub fn difference(&self, other: &Self) -> i32 {
        other.total() - self.total()
    }

}

/// LeetCode company workers use key-cards to unlock office doors. Each time a worker uses their
/// key-card, the security system saves the worker's name and the time when it was used. The system
/// emits an alert if any worker uses the key-card three or more times in a one-hour period.
///
/// You are given a list of strings `keyName` and `keyTime` where `[keyName[i], keyTime[i]]`
/// corresponds to a person's name and the time when their key-card was used in a single day.
///
/// Access times are given in the 24-hour time format "HH:MM", such as "23:51" and "09:49". Return
/// a list of unique worker names who received an alert for frequent keycard use. Sort the names in
/// ascending order alphabetically.
///
/// Notice that "10:00" - "11:00" is considered to be within a one-hour period, while "22:51" -
/// "23:52" is not considered to be within a one-hour period.
struct Solution;

impl Solution {

    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut times_by_name = HashMap::new();
        let n = key_name.len();

        for i in 0..n {
            let name = &key_name[i];
            let time = Time::parse(&key_time[i]);

            times_by_name
                .entry(name)
                .or_insert(Vec::new())
                .push(time);
        }

        let mut result = Vec::new();

        for (name, mut values) in times_by_name {
            let n = values.len();
            values.sort_unstable();

            for i in 0..n {
                if i+2 < n {
                    let current = values[i];
                    let ahead = values[i+2];
                    if current.difference(&ahead) <= 60 {
                        result.push(name.to_string());
                        break;
                    }
                }
            }

        }

        result.sort_unstable();
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let key_name = vec!["daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"];
        let key_name = key_name.into_iter().map(|s| s.to_string()).collect();
        let key_time = vec!["10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00"];
        let key_time = key_time.into_iter().map(|s| s.to_string()).collect();
        let result = Solution::alert_names(key_name, key_time);
        assert_eq!(result, vec!["daniel"]);
    }

    #[test]
    fn example_2() {
        let key_name = vec!["alice", "alice", "alice", "bob", "bob", "bob", "bob"];
        let key_name = key_name.into_iter().map(|s| s.to_string()).collect();
        let key_time = vec!["12:01", "12:00", "18:00", "21:00", "21:20", "21:30", "23:00"];
        let key_time = key_time.into_iter().map(|s| s.to_string()).collect();
        let result = Solution::alert_names(key_name, key_time);
        assert_eq!(result, vec!["bob"]);
    }

    #[test]
    fn example_3() {
        let key_name = vec!["daniel".to_string()];
        let key_time = vec!["10:00".to_string()];
        let result = Solution::alert_names(key_name, key_time);
        assert_eq!(result, Vec::<String>::new());
    }

}
