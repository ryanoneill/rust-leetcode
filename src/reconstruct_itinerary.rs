use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Ticket {
    from: String,
    to: String,
}

impl Ticket {

    fn new(from: String, to: String) -> Self {
        Self { from, to }
    }

    fn from_vec(ticket: &Vec<String>) -> Self {
        Self { from: ticket[0].clone(), to: ticket[1].clone() }
    }

}

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

    fn to_map(tickets: &Vec<Vec<String>>) -> HashMap<String, HashSet<Ticket>> {
        let mut result = HashMap::new();
        for item in tickets {
            let ticket = Ticket::from_vec(item);
            let key = ticket.from.clone();
            result
                .entry(key)
                .or_insert(HashSet::new())
                .insert(ticket);
        }

        result
    }

    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut results = Vec::new();

        let tickets = Self::to_map(&tickets);
        let city = "JFK".to_string();
        let path = vec![];
        let seen = HashSet::new();
        Self::worker(&tickets, city, path, seen, &mut results);

        // TODO: Take lexical order result
        vec![]
    }

    // TODO: Finish Implementation
    fn worker(
        _tickets: &HashMap<String, HashSet<Ticket>>,
        _city: String,
        _path: Vec<String>,
        _seen: HashSet<Ticket>,
        _results: &mut Vec<Vec<String>>
    ) {

    }

    // pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    //     let mut results = Vec::new();
    //     Self::worker(&graph, 0, vec![], &mut results);
    //     results
    // }

    // fn worker(graph: &Vec<Vec<i32>>, node: i32, path: Vec<i32>, results: &mut Vec<Vec<i32>>) {
    //     let n = graph.len();
    //     let i = node as usize;
    //     let mut path = path;
    //     path.push(node);
    //     if i == n - 1 {
    //         results.push(path);
    //     } else {
    //         for &next in &graph[i] {
    //             let cloned_path = path.clone();
    //             Self::worker(graph, next, cloned_path, results);
    //         }
    //     }
    // }

}
