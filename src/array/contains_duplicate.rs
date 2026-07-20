use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::with_capacity(nums.len());

    for num in &nums {
        if seen.contains(&num) {
            return true;
        }
        seen.insert(num);
    }
    false
}
