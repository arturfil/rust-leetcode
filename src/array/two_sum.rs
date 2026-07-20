use std::collections::HashMap;


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());

    for (i, num) in nums.iter().enumerate() {
        let target_val = target - num;

        if let Some(&other_num) = map.get(&target_val) {
            return vec![other_num, i as i32];
        }
        map.insert(*num, i as i32);

    }
    vec![]
}
