
pub fn climb_stairs(n: i32) -> i32 {
    if n == 1 { return 1};
    if n == 2 { return 2};

    let n = n as usize;
    let mut ways = vec![1; n + 1];
    (ways[0], ways[1], ways[2]) = (1, 1,  2);

    for i in 1..=n {
        ways[i] = ways[i-1] + ways[i-2];
    } 

    ways[n]
}
