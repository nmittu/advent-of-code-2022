use std::{fs::File, io::{BufReader, BufRead}, env, cmp::{min, max}};


fn get_pairs(file: File) -> impl Iterator<Item = ((i32, i32), (i32, i32))> {
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| 
            l.split(",").map(|r| 
                r.split("-").map(|s| 
                    s.parse::<i32>().unwrap()
                ).collect()
            ).collect::<Vec<Vec<i32>>>())
        .map(|p| ((p[0][0], p[0][1]), (p[1][0], p[1][1])))
}

fn is_contained(r: &((i32, i32), (i32, i32))) -> bool {
    let (r1, r2) = *r;
    (r1.0 <= r2.0 && r1.1 >= r2.1) || ((r1.0 >= r2.0 && r1.1 <= r2.1))
}

fn pt1(file: File) -> i32 {
    get_pairs(file).filter(is_contained).count() as i32
}

fn is_overlaping(r: &((i32, i32), (i32, i32))) -> bool {
    let (r1, r2) = *r;
    max(r1.0, r2.0) <= min(r1.1, r2.1)
}

fn pt2(file: File) -> i32 {
    get_pairs(file).filter(is_overlaping).count() as i32
}

fn main() {
    let file_name = &env::args().collect::<Vec<String>>()[1];

    let file = File::open(file_name).expect("Could not open file");
    println!("pt1: {}", pt1(file));

    let file = File::open(file_name).expect("Could not open file");
    println!("pt2: {}", pt2(file));

}