use std::{env, fs::File, io::{BufReader, BufRead}};

fn pt1(file: File) -> i32 {
    let reader = BufReader::new(file);

    let mut max = 0;
    let mut cur = 0;

    for line in reader.lines() {
        let calories = line.unwrap().parse::<i32>();

        match calories {
            Ok(c) => cur += c,
            Err(_) => {
                if cur > max {
                    max = cur;
                }
                cur = 0;
            },
        }
    }

    max
}

fn insert(arr: &mut [i32], x: i32) {
    let mut i = 0;

    while i < arr.len() {
        if arr[i] < x {
            arr[i..].rotate_right(1);
            arr[i] = x;
            return;
        }
        i+=1;
    }
}

fn pt2(file: File) -> i32 {
    let reader = BufReader::new(file);

    let mut maxes = [0,0,0];
    let mut cur = 0;

    for line in reader.lines() {
        let calories = line.unwrap().parse::<i32>();

        match calories {
            Ok(c) => cur += c,
            Err(_) => {
                insert(&mut maxes, cur);
                cur = 0;
            },
        }
    }


    maxes.iter().sum()
}

fn main() {
    let file_name = &env::args().collect::<Vec<String>>()[1];

    let file = File::open(file_name).expect("Could not open file");
    println!("pt1: {}", pt1(file));

    let file = File::open(file_name).expect("Could not open file");
    println!("pt2: {}", pt2(file));

}