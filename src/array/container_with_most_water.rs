use std::cmp::{max, min};

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut area:i32 = 0;
    let (mut l, mut r) = (0, height.len() -1);

    while l < r {
        let base = (r - l) as i32;
        let min_height = min(height[l], height[r]);
        let cur_ar = base * min_height;

        area = max(area, cur_ar);

        if height[l] > height[r] { r -= 1; }
        else { l += 1; }
    }
        
    area
}

