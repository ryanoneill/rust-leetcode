use std::collections::HashMap;
use std::collections::HashSet;

/// You are given a list of airline `tickets` where `tickets[i] = [fromi, toi]`
/// represent the departure and arrival airports of one flight. Reconstruct
/// the itinerary in order and return it.
///
/// All of the tickets belong to a man who departs from `"JFK"`, thus, the
/// itinerary must begin with `"JFK"`. If there are multiple valid itineraries,
/// you should retun the itinerary that has the smallest lexical order when
/// read as a single string.
///
/// * For example, the itinerary `["JFK", "LGA"]` has a smaller lexical order
/// than `["JFK", "LGB"]`.
///
/// You may assume all tickets form at least one valid itinerary. You must use
/// all the tickets once and only once.
struct Solution;

impl Solution {

    fn to_adj_map(tickets: Vec<Vec<String>>) -> HashMap<String, HashSet<String>> {
        let mut results = HashMap::new();

        for ticket in tickets {
            results
                .entry(ticket[0].clone())
                .or_insert(HashSet::new())
                .insert(ticket[1].clone());
        }

        results
    }

    fn remove(map: &mut HashMap<String, HashSet<String>>, key: &String, value: &String) {
        if map.contains_key(key) {
            let set = map.get_mut(key).unwrap();
            if set.len() == 1 {
                map.remove(key);
            } else {
                set.remove(value);
            }
        }
    }

    fn worker(
        results: &mut Vec<String>,
        map: HashMap<String, HashSet<String>>,
        path: Vec<String>,
        current: String
    ) {
        if map.len() == 0 {
            if results.len() == 0 {
                *results = path.clone();
            } else if &path < &results{
                *results = path.clone();
            }
        } else if map.contains_key(&current) {
            for next in &map[&current] {
                let mut next_path = path.clone();
                next_path.push(next.clone());
                let mut map = map.clone();
                Self::remove(&mut map, &current, next);
                Self::worker(results, map, next_path, next.clone());
            }
        }
    }

    // TODO: Improve
    // Simple solution that I know if submitted will fail for time limit exceeded.
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut results = Vec::new();

        let map = Self::to_adj_map(tickets);
        let initial = vec!["JFK".to_string()];
        let current = "JFK".to_string();

        Self::worker(&mut results, map, initial, current);

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        let tickets = vec![
            vec!["MUC".to_string(), "LHR".to_string()],
            vec!["JFK".to_string(), "MUC".to_string()],
            vec!["SFO".to_string(), "SJC".to_string()],
            vec!["LHR".to_string(), "SFO".to_string()],
        ];
        let result = Solution::find_itinerary(tickets);
        assert_eq!(result, vec!["JFK", "MUC", "LHR", "SFO", "SJC"]);
    }

    #[ignore]
    #[test]
    fn example_2() {
        let tickets = vec![
            vec!["JFK".to_string(), "SFO".to_string()],
            vec!["JFK".to_string(), "ATL".to_string()],
            vec!["SFO".to_string(), "ATL".to_string()],
            vec!["ATL".to_string(), "JFK".to_string()],
            vec!["ATL".to_string(), "SFO".to_string()],
        ];
        let result = Solution::find_itinerary(tickets);
        assert_eq!(result, vec!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]);
    }

}
