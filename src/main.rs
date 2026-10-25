mod heaps;

use heaps::top_k_f_elements::top_k_frequent;

fn main() {
    let res = top_k_frequent(vec![1,1,1,3,3,1,2,4,6], 2);
    println!("{:?}", res);
}
