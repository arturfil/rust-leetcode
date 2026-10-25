use std::{ cmp::Reverse, collections::{BinaryHeap, HashMap}};

// OPTION 1
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut n_map: HashMap<i32, i32> = HashMap::new();

    for n in nums {
        *n_map.entry(n).or_default() += 1;
    }

    let mut heap:BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();

    for (key, val) in n_map {
        heap.push(Reverse((val, key)));
        if heap.len() > k as usize {
            heap.pop();
        }
    }

    heap.into_iter().map(|Reverse((_, key))| key).collect()
}

/* OPTION 2
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut nums_heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    let mut n_map: HashMap<i32, i32> = HashMap::new();
    let mut res: Vec<i32> = Vec::new();

    for n in &nums {
        *n_map.entry(*n).or_insert(0) += 1;
    }

    for (k, v) in &n_map {
        nums_heap.push((*v, *k));
    }

    for _ in 1..=k {
        if let Some((fq, key)) = nums_heap.pop() {
            res.push(key);
        }
    }

    res
}
*/
