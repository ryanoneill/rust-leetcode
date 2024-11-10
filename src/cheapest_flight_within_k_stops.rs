use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Flight {
    src: i32,
    dst: i32,
    cost: i32,
}

impl Flight {

    pub fn new(src: i32, dst: i32, cost: i32) -> Self {
        Self {
            src,
            dst,
            cost
        }
    }

}

/// There are `n` cities connected by some number of flights. You are given an array `flights`
/// where `flights[i] = [fromi, toi, pricei]` indicates that there is a flight from city `fromi` to
/// city `toi` with cost `pricei`.
///
/// You are also given three integers `src`, `dst`, and `k`, return the cheapest price from `src`
/// to `dst` with at most `k` stops. If there is no such route, return `-1`.
struct Solution;

impl Solution {

    fn to_flight_map(flights: Vec<Vec<i32>>) -> HashMap<i32, Vec<Flight>> {
        let mut result: HashMap<i32, Vec<Flight>> = HashMap::new();

        for item in flights {
            let flight = Flight::new(item[0], item[1], item[2]);
            result
                .entry(item[0])
                .and_modify(|fs| fs.push(flight))
                .or_insert(vec![flight]);
        }

        result
    }

    pub fn find_cheapest_price(_n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut airports = k + 1;

        let flight_map = Self::to_flight_map(flights);
        let mut cheapest = HashMap::new();

        let mut queue = VecDeque::new();
        queue.push_back((src, 0));

        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let (dst, cost) = queue.pop_front().unwrap();
                if !cheapest.contains_key(&dst) || cost < cheapest[&dst] {
                    cheapest.insert(dst, cost);
                    if flight_map.contains_key(&dst) {
                        for leg in &flight_map[&dst] {
                            queue.push_back((leg.dst, leg.cost + cost));
                        }
                    }
                }
            }

            airports -= 1;
            if airports < 0 {
                break;
            }
        }
        
        if cheapest.contains_key(&dst) {
            cheapest[&dst]
        } else {
            -1
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 4;
        let flights = vec![
            vec![0,1,100],
            vec![1,2,100],
            vec![2,0,100],
            vec![1,3,600],
            vec![2,3,200],
        ];
        let src = 0;
        let dst = 3;
        let k = 1;
        let result = Solution::find_cheapest_price(n, flights, src, dst, k);
        assert_eq!(result, 700);
    }

    #[test]
    fn example_2() {
        let n = 3;
        let flights = vec![
            vec![0,1,100],
            vec![1,2,100],
            vec![0,2,500],
        ];
        let src = 0;
        let dst = 2;
        let k = 1;
        let result = Solution::find_cheapest_price(n, flights, src, dst, k);
        assert_eq!(result, 200);
    }

    #[test]
    fn example_3() {
        let n = 3;
        let flights = vec![
            vec![0,1,100],
            vec![1,2,100],
            vec![0,2,500],
        ];
        let src = 0;
        let dst = 2;
        let k = 0;
        let result = Solution::find_cheapest_price(n, flights, src, dst, k);
        assert_eq!(result, 500);
    }

}
