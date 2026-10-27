mod intervals;

use intervals::merge_intervals::merge;

/*
Input: intervals = []
*/

fn main() {
    merge(vec![vec![2,6], vec![1,3],vec![8,10],vec![15,18]]);
}
