use std::{collections::HashMap, hash::Hash};

/// You are given a 2D integer array `logs` where each
/// `logs[i] = [birthi, deathi]` indicates the birth and death year of the
/// `ith` person.
///
/// The population of some year `x` is the number of people alive during that
/// year. The `ith` persion is counted in year `x`'s population if `x` is in
/// the inclusive range `[birthi, deathi - 1]`. Note that the person is not
/// counted in the year that they die.
///
/// Return the earliest year with the maximum population.
struct Solution;

impl Solution {

    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut first_birth: Option<i32> = None;
        let mut last_death: Option<i32> = None;
        let mut births: HashMap<i32, usize> = HashMap::new();
        let mut deaths: HashMap<i32, usize> = HashMap::new();

        for log in logs {
            let birth = log[0];
            let death = log[1];

            if first_birth.is_none() { first_birth = Some(birth); }
            else { first_birth.map(|fb| if birth < fb { first_birth = Some(birth) }); }

            if last_death.is_none() { last_death = Some(death); }
            else { last_death.map(|ld| if death > ld { last_death = Some(death) }); }

            births.entry(birth).and_modify(|count| *count += 1).or_insert(1);
            deaths.entry(death).and_modify(|count| *count += 1).or_insert(1);
        }

        let first = first_birth.unwrap();
        let last = last_death.unwrap();

        let mut result = 0;
        let mut max_population = 0;
        let mut population = 0;
        let mut init = true;

        for year in first..=last {
            let additions = births.get(&year);
            additions.map(|a| population += *a as i32);
            let subtractions = deaths.get(&year);
            subtractions.map(|s| population -= *s as i32);
            assert!(population >= 0);

            if init {
                max_population = population;
                result = year;
                init = false;
            } else if population > max_population {
                max_population = population;
                result = year;
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
        let logs = vec![vec![1993, 1999], vec![2000, 2010]];
        let result = Solution::maximum_population(logs);
        assert_eq!(result, 1993);
    }

    #[test]
    fn example_2() {
        let logs = vec![vec![1950, 1961], vec![1960, 1971], vec![1970, 1981]];
        let result = Solution::maximum_population(logs);
        assert_eq!(result, 1960);
    }

}
