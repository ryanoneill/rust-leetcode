use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, Eq, PartialEq, Hash)]
struct BoxType {
    units_per_box: i32,
    number_of_boxes: i32,
}

impl BoxType {

    pub fn new(units_per_box: i32, number_of_boxes: i32) -> Self {
        Self { units_per_box, number_of_boxes }
    }

    pub fn total_units(&self) -> i32 {
        self.units_per_box * self.number_of_boxes
    }

}

/// You are assigned to put some amount of boxes onto one truck. You are given a
/// 2D array `boxTypes`, where
/// `boxTypes[i] = [numberOfBoxesi, numberOfUnitsPerBoxi]`:
///
/// * `numberOfBoxesi` is the number of boxes of type `i`.
///
/// * `numberOfUnitsPerBoxi` is the number of units in each box of the type `i`.
///
/// You are also given an integer `truckSize`, which is the maximum number of
/// boxes that can be put on the truck. You can choose any boxes to put on the
/// truck as long as the number of boxes does not exceed `truckSize`.
///
/// Return the maximum total number of units that can be put on the truck.
struct Solution;

impl Solution {

    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut box_heap = BinaryHeap::new();

        for values in box_types {
            let units_per_box = values[1];
            let number_of_boxes = values[0];
            let box_type = BoxType::new(units_per_box, number_of_boxes);
            box_heap.push(box_type);
        }

        let mut size = truck_size;
        let mut result = 0;

        while !box_heap.is_empty() {
            let box_type = box_heap.pop().unwrap();

            if size > box_type.number_of_boxes {
                result += box_type.total_units();
                size -= box_type.number_of_boxes;
            } else {
                result += size * box_type.units_per_box;
                break;
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
        let box_types = vec![vec![1,3], vec![2,2], vec![3,1]];
        let truck_size = 4;
        let result = Solution::maximum_units(box_types, truck_size);
        assert_eq!(result, 8);
    }

    #[test]
    fn example_2() {
        let box_types = vec![vec![5,10], vec![2,5], vec![4,7], vec![3,9]];
        let truck_size = 10;
        let result = Solution::maximum_units(box_types, truck_size);
        assert_eq!(result, 91);
    }

}
