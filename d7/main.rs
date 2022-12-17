use std::{io::{Lines, BufReader, BufRead}, fs::File, iter::{from_fn, Peekable}, env, cmp::min};

const MAX_THREAS: u64 = 100000;
const FS_SIZE: u64 = 70000000;
const NEEDED: u64 = 30000000;

struct Dir(Vec<Dir>, u64);

fn sh(lines: &mut Peekable<Lines<BufReader<File>>>) -> Dir {
    let mut dirs = vec![];
    let mut size = 0;


    while let Some(Ok(s)) = lines.next() {
        if s == "$ cd .." {
            break;
        } else if s == "$ ls" {
            size = from_fn(|| lines.next_if(|s| {!s.as_ref().unwrap().starts_with("$")}))
                        .map(|s| s.unwrap())
                        .filter(|s| !s.starts_with("dir"))
                        .map(|s| s.split_ascii_whitespace().next().unwrap().parse::<u64>().unwrap())
                        .sum::<u64>();
        } else {
            let tmp_dir = sh(lines);
            size += tmp_dir.1;
            dirs.push(tmp_dir);

        }
    } 

    Dir(dirs, size)
}

fn search(dir: &Dir) -> u64 {
    (if dir.1 <= MAX_THREAS { dir.1 } else {0}) + dir.0.iter().map(|d| search(d)).sum::<u64>()
}

fn pt1(file: File) -> u64 {
    let mut lines = BufReader::new(file).lines().peekable();
    let dir = sh(&mut lines);

    search(&dir)
}

fn to_delete(dir: &Dir, needed: u64) -> u64 {
    let candidate = dir.0.iter().map(|d| to_delete(d, needed)).min();

    match candidate {
        Some(s) => if dir.1 >= needed {
            min(dir.1, s)
        } else {
            s
        },
        None => if dir.1 >= needed {
            dir.1
        } else {
            FS_SIZE
        }
    }
}

fn pt2(file: File) -> u64 {
    let mut lines = BufReader::new(file).lines().peekable();
    let dir = sh(&mut lines);

    to_delete(&dir, NEEDED - (FS_SIZE - dir.1))
}


fn main() {
    let file_name = &env::args().collect::<Vec<String>>()[1];

    let file = File::open(file_name).expect("Could not open file");
    println!("pt1: {}", pt1(file));
    
    let file = File::open(file_name).expect("Could not open file");
    println!("pt2: {}", pt2(file));

}