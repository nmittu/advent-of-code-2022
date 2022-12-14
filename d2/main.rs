use std::{fs::File, io::{BufReader, BufRead}, env};

fn calc_score(m1: &str, m2: &str) -> i32 {
    match (m1, m2) {
        ("A", "X") => 4,
        ("A", "Y") => 8,
        ("A", "Z") => 3,

        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        
        ("C", "X") => 7,
        ("C", "Y") => 2,
        ("C", "Z") => 6,

        _ => panic!("Invalid move"),
    }
}

fn pt1(file: File) -> i32 {
    let reader = BufReader::new(file);

    let mut score = 0;

    for line in reader.lines() {
        let line =line.unwrap();
        if let [m1, m2] = line.split(" ").collect::<Vec<&str>>()[..] {
            score += calc_score(m1, m2);
        }
    }

    score
}

fn calc_score_2(m1: &str, m2: &str) -> i32 {
    match (m1, m2) {
        ("A", "X") => 3,
        ("A", "Y") => 4,
        ("A", "Z") => 8,

        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        
        ("C", "X") => 2,
        ("C", "Y") => 6,
        ("C", "Z") => 7,

        _ => panic!("Invalid move"),
    }
}

fn pt2(file: File) -> i32 {
    let reader = BufReader::new(file);

    let mut score = 0;

    for line in reader.lines() {
        let line =line.unwrap();
        if let [m1, m2] = line.split(" ").collect::<Vec<&str>>()[..] {
            score += calc_score_2(m1, m2);
        }
    }

    score
}


fn main() {
    let file_name = &env::args().collect::<Vec<String>>()[1];

    let file = File::open(file_name).expect("Could not open file");
    println!("pt1: {}", pt1(file));
    
    let file = File::open(file_name).expect("Could not open file");
    println!("pt2: {}", pt2(file));
}