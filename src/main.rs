mod working;

use working::test::max_area;


fn main() {
    let res = max_area(vec![1,7,2,3,5,3]); // 5 * 3 = 15

    println!("res -> {}", res);
}
