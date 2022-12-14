use std::{env, fs::File, io::{BufReader, BufRead}, collections::HashSet};
use itertools::Itertools;

fn priority(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        (c as i32) - ('a' as i32) + 1
    } else {
        (c as i32) - ('A' as i32) + 27
    }
}

fn pt1(file: File) -> i32 {
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| l.unwrap())
        .fold(0, |sum, l| {
            let (l, r) = (&l[..l.len()/2], &l[l.len()/2..]);

            let l_set = l.chars().collect::<HashSet<char>>();
            let r_set = r.chars().collect::<HashSet<char>>();

            match r_set.intersection(&l_set).nth(0) {
                Some(c) => sum + priority(*c),
                None => panic!("No common chars")
            }
        })
}

fn pt2(file: File) -> i32 {
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| l.unwrap())
        .chunks(3)
        .into_iter()
        .fold(0, |s, c| {
            let (mut b1, b2, b3) = c.map(|l| l.chars().collect::<HashSet<char>>())
                                .collect_tuple().unwrap();

            b1.retain(|c| b2.contains(c) && b3.contains(c));

            match b1.into_iter().nth(0) {
                Some(c) => s + priority(c),
                None => panic!("No common chars"),
            }
        })

}

fn main() {
    let file_name = &env::args().collect::<Vec<String>>()[1];

    let file = File::open(file_name).expect("Could not open file");
    println!("pt1: {}", pt1(file));

    let file = File::open(file_name).expect("Could not open file");
    println!("pt2: {}", pt2(file));
}