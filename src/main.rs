mod working;

use working::word_search::exists;

fn main() {

    let matrix: Vec<Vec<char>> = vec![
        vec!['A', 'B', 'C', 'D'],
        vec!['B', 'B', 'B', 'B'],
        vec!['D', 'D', 'C', 'O'],
        vec!['E', 'E', 'E', 'I'],
    ];

    let res = exists(matrix, "ABCD".to_string());
    println!("{}", res);
}


