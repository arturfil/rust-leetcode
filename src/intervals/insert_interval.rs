pub fn insert_interval(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();

    println!();
    for interval in &intervals {
        print!("[{}, {}],", interval[0], interval[1]);

        res.push(interval.to_vec())
    }

    println!("\n{:?}", res);

    intervals
}
