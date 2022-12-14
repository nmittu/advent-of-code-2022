use std::{fs::{self}, collections::HashSet, env};

fn solution(filepath: &str, len: usize) -> usize {
    let s = fs::read_to_string(filepath).expect("Could not read file");

    for i in 0..s.len()-len {
        if s[i..i+len].chars().collect::<HashSet<char>>().len() == len {
            return i+len;
        }
    }

    0
}

fn main() {
    let file_name = &env::args().collect::<Vec<String>>()[1];

    println!("pt1: {}", solution(file_name, 4));
    println!("pt1: {}", solution(file_name, 14));
}