mod helpers;

use helpers::select_random::{select_random_problem, Category, Difficulty};

fn main() {
    select_random_problem(None, Some(Difficulty::Medium));

}


