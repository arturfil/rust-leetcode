pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![1; nums.len()];
    let mut prev = 1;

    for i in 0..nums.len() {
        res[i] = prev;
        prev *= nums[i];
    }

    let mut prv = 1;
    for i in (0..res.len()).rev() {
        res[i] *= prv;
        prv *= nums[i];
    }

    res
}
