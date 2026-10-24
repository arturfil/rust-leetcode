mod array;

use array::group_anagrams::group_anagrams;

fn main() {
    group_anagrams(vec![
        "ate".to_string(),
        "tae".to_string(),
        "tea".to_string(),
        "eat".to_string(),
        "tar".to_string(),
        "rat".to_string(),
        "pat".to_string(),
    ]);
}
