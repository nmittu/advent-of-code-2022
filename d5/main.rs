use std::{fs::File, io::{BufReader, BufRead, Lines}, env};
use regex::Regex;

fn read_init(lines: &mut Lines<BufReader<File>>) -> Vec<Vec<char>> {
    let mut buffer = vec![];

    
    let num_vec;
    loop {
        let l = lines.next().unwrap().unwrap();

        if l.len() < 2 || l.chars().nth(1).unwrap() != '1' {
            buffer.push(l);
        } else if l.len() >= 2 {
            num_vec = l.split(" ").filter(|s| s.len() > 0).count();
            break;
        }
    }

    // Read empty line
    lines.next();

    let mut stacks = (0..num_vec).map(|_| vec![]).collect::<Vec<Vec<char>>>();

    for row in buffer.iter().rev() {
        let mut chars = row.chars().collect::<Vec<char>>();

        let expected_len = num_vec * 4 - 1;

        if chars.len() < expected_len {
            chars.extend((chars.len()..expected_len).map(|_| ' '));
        }

        for i in 0..num_vec {
            if chars[i*4 + 1] != ' ' {
                stacks[i].push(chars[i*4 + 1]);
            }
        }
    }

    stacks
}

fn get_instructions(lines: Lines<BufReader<File>>) -> impl Iterator<Item = (usize, usize, usize)> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    
    lines
        .map(|l| l.unwrap())
        .map(move |l| {
            let captures = re.captures(&l).expect("invalid line");

            let num = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let to = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();

            (num, from, to)
        })

} 

fn pt1(file: File) -> String {
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let mut stacks = read_init(&mut lines);

    for (num, from, to) in get_instructions(lines) {
        for _ in 0..num {
            let c = stacks[from-1].pop().unwrap();
            stacks[to-1].push(c);
        }
    }

    let mut result = String::new();

    for s in stacks {
        let c = s.last();
        if c.is_some() {
            result.push(*c.unwrap());
        }
    }

    result
}

fn pt2(file: File) -> String {
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let mut stacks = read_init(&mut lines);

    for (num, from, to) in get_instructions(lines) {
        let from_stack = &mut stacks[from - 1];
        let chrs = from_stack.drain(from_stack.len()-num..).collect::<Vec<char>>();
        stacks[to-1].extend(chrs);
    }

    let mut result = String::new();

    for s in stacks {
        let c = s.last();
        if c.is_some() {
            result.push(*c.unwrap());
        }
    }

    result
}

fn main() {
    let file_name = &env::args().collect::<Vec<String>>()[1];

    let file = File::open(file_name).expect("Could not open file");
    println!("pt1: {}", pt1(file));
    
    let file = File::open(file_name).expect("Could not open file");
    println!("pt2: {}", pt2(file));
}