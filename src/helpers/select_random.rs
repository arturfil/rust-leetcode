use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

pub enum Category {
    TwoPointers,
    Math,
    String,
    Array,
    BinarySearch,
    BitManipulation,
    DynamicProgramming,
    BackTracking,
    Greedy,
    Graph,
    Intervals,
    LinkedLists,
    Matrix,
    Backtracking,
    SlidingWindow,
    Stack,
    BinaryTree,
    Trie,
    Heap,
    Design,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::TwoPointers => "two_pointers",
            Category::Math => "math",
            Category::String => "string",
            Category::Array => "array",
            Category::BinarySearch => "binary_search",
            Category::BitManipulation => "bit_manipulation",
            Category::DynamicProgramming => "dynamic_programming",
            Category::BackTracking => "backtracking",
            Category::Greedy => "greedy",
            Category::Graph => "graph",
            Category::Intervals => "intervals",
            Category::LinkedLists => "linked_lists",
            Category::Matrix => "matrix",
            Category::Backtracking => "back_tracking",
            Category::SlidingWindow => "sliding_window",
            Category::Stack => "stack",
            Category::BinaryTree => "binary_tree",
            Category::Trie => "trie",
            Category::Heap => "heap",
            Category::Design => "design",
        }
    }
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Difficulty::Easy => "easy",
            Difficulty::Medium => "medium",
            Difficulty::Hard => "hard",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    // would be like classes in OOP
    done: bool,
    title: String,
    category: String,
    difficulty: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    problems: Vec<Problem>,
}

pub fn select_random_problem(category: Option<Category>, difficulty: Option<Difficulty>) {
    let data = fs::read_to_string("./data.json").expect("Unable to read file"); // read json file
    let data_obj: Data = serde_json::from_str(&data).expect("ERROR: Should read the file");

    let not_done: Vec<&Problem> = data_obj
        .problems
        .iter()
        .filter(|p| !p.done)
        .filter(|p| category.as_ref().map_or(true, |c| p.category == c.as_str()))
        .filter(|p| difficulty.as_ref().map_or(true, |d| p.difficulty == d.as_str()))
        .collect();

    match not_done.choose(&mut rand::thread_rng()) {
        Some(p) => println!("\nProblem Chose ---> {:?}", p.title),
        None => println!("\nNo matching problems left."),
    }
}

/*
pub fn select_random_problem_forloop(category: Option<Category>, difficulty: Option<Difficulty>) {
    let data = fs::read_to_string("./data.json").expect("Unable to read file"); // read json file 
    let data_obj: Data = serde_json::from_str(&data).expect("ERROR: Should read the file"); // parse json to vector 

    let mut not_done: Vec<Problem> = Vec::new(); // create vector


    for prb in data_obj.problems.iter() {

        if let Some(c) = &category {
            if c.as_str() != prb.category.to_string() {
                continue;
            }
        }

        if let Some(d) = &difficulty {
            if d.as_str() != prb.difficulty.to_string() {
                continue;
            }
        }
        // add "not done" to vector
        if prb.done == false {
            not_done.push(Problem {
                done: prb.done,
                title: prb.title.to_string(),
                category: prb.category.to_string(),
                difficulty: prb.difficulty.to_string(),
            });
        }
    }

    let mut rng = rand::thread_rng(); // generate random seed
    let rand: usize = rng
        .gen_range(0..not_done.len()) // build random uint
        .try_into()
        .unwrap();
    println!(
        "\nProblem Chosen ---> {:?}",
        not_done[rand].title.to_string()
    );
}
*/
