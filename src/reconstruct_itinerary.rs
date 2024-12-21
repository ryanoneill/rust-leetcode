use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

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

    fn dfs(
        result: &mut Vec<String>,
        adj_map: &mut HashMap<String, BinaryHeap<Reverse<String>>>,
        value: &str
    ) {
        while let Some(Reverse(next)) = adj_map.get_mut(value).and_then(|dests| dests.pop()) {
            Self::dfs(result, adj_map, &next);
        }
        result.push(value.to_string());
    }

    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut result = vec![];
        let mut adj_map: HashMap<String, BinaryHeap<Reverse<String>>> = HashMap::new();

        for ticket in tickets {
            let source = ticket[0].clone();
            let destination = ticket[1].clone();

            adj_map
                .entry(source)
                .or_insert(BinaryHeap::new())
                .push(Reverse(destination));
        }

        Self::dfs(&mut result, &mut adj_map, "JFK");

        result.reverse();
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

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

    #[test]
    fn example_3() {
        let tickets = vec![
            vec![str!("EZE"), str!("AXA")],
            vec![str!("TIA"), str!("ANU")],
            vec![str!("ANU"), str!("JFK")],
            vec![str!("JFK"), str!("ANU")],
            vec![str!("ANU"), str!("EZE")],
            vec![str!("TIA"), str!("ANU")],
            vec![str!("AXA"), str!("TIA")],
            vec![str!("TIA"), str!("JFK")],
            vec![str!("ANU"), str!("TIA")],
            vec![str!("JFK"), str!("TIA")],
        ];
        let result = Solution::find_itinerary(tickets);
        assert_eq!(result, vec!["JFK","ANU","EZE","AXA","TIA","ANU","JFK","TIA","ANU","TIA","JFK"]);
    }

}
