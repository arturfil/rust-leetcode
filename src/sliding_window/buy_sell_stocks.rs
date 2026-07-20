use std::cmp::max;


pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut prof: i32 = -1; 
    let mut min:i32 = prices[0];

    for p in &prices {
        if *p < min {
            min = *p;
        } else {
            prof = max(prof, *p - min);
        }
    }

    prof
}
