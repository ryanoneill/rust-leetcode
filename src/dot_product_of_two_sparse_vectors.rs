use std::collections::HashMap;
use std::collections::HashSet;

/// Given two sparse vectors, compute their dot product.
///
/// Implement class `SparseVector`:
///
/// * `SparseVector(nums)` Initializes the object with the vector `nums`
/// * `dotProduct(vec)` Compute the dot product between the instance of SparseVector and `vec`
///
/// A sparse vector is a vector that has mostly zero values, you should store the spare vector
/// efficiently and compute the dot product between two SparseVector.
struct SparseVector {
    items: HashMap<usize, i32>,
}

impl SparseVector {
    
    pub fn new(nums: Vec<i32>) -> Self {
        let mut items = HashMap::new();
        let n = nums.len();
        for i in 0..n {
            let num = nums[i];
            if num != 0 {
                items.insert(i, num);
            }
        }
        Self { items }
    }

    // LeetCode defines this method as taking in `vec: SparseVector`. There's
    // no reason for the `dot_product` method to take ownership of the second
    // `SparseVector` and it should be `vec: &SparseVector` in my opinion.
    fn dot_product(&self, vec: SparseVector) -> i32 {
        let keys1: HashSet<usize> = self.items.keys().copied().collect();
        let keys2: HashSet<usize> = vec.items.keys().copied().collect();
        let both = keys1.intersection(&keys2);

        let mut result = 0;

        for key in both {
            let first = self.items[&key];
            let second = vec.items[&key];
            let product = first * second;
            result += product;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::SparseVector;

    #[test]
    fn example_1() {
        let nums1 = vec![1,0,0,2,3];
        let nums2 = vec![0,3,0,4,0];
        let v1 = SparseVector::new(nums1);
        let v2 = SparseVector::new(nums2);
        let result = v1.dot_product(v2);
        assert_eq!(result, 8);
    }

    #[test]
    fn example_2() {
        let nums1 = vec![0,1,0,0,0];
        let nums2 = vec![0,0,0,0,2];
        let v1 = SparseVector::new(nums1);
        let v2 = SparseVector::new(nums2);
        let result = v1.dot_product(v2);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let nums1 = vec![0,1,0,0,2,0,0];
        let nums2 = vec![1,0,0,0,3,0,4];
        let v1 = SparseVector::new(nums1);
        let v2 = SparseVector::new(nums2);
        let result = v1.dot_product(v2);
        assert_eq!(result, 6);
    }

}

// 1. Write down the problem
