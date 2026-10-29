
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0])); // sort asc [ [1, 3], [5, 6], ...]
    let mut res = vec![intervals[0].clone()]; // <- first interval inside res var
    
    for interval in &intervals[1..] {

        if let Some(last) = res.last_mut() {
            if last[1] >= interval[0] {
                last[1] = last[1].max(interval[1]) // max of last[1], interval[1]
            } else {
                res.push(interval.to_vec());
            }
        }
    }
    res
}

/*
Testing:
merge(vec![vec![2,6], vec![1,3],vec![8,10],vec![15,18]]);
*/
