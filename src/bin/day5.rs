use std::fs::File;
use std::io::{BufRead, BufReader};

struct Move {
    num: i32,
    source: usize,
    dest: usize,
}

fn initialize_stacks() -> [Vec<char>; 9] {
    [
        vec!['H', 'C', 'R'],
        vec!['B', 'J', 'H', 'L', 'S', 'F'],
        vec!['R', 'M', 'D', 'H', 'J', 'T', 'Q'],
        vec!['S', 'G', 'R', 'H', 'Z', 'B', 'J'],
        vec!['R', 'P', 'F', 'Z', 'T', 'D', 'C', 'B'],
        vec!['T', 'H', 'C', 'G'],
        vec!['S', 'N', 'V', 'Z', 'B', 'P', 'W', 'L'],
        vec!['R', 'J', 'Q', 'G', 'C'],
        vec!['L', 'D', 'T', 'R', 'H', 'P', 'F', 'S'],
    ]
}

fn print_stack_tops(stacks: &[Vec<char>; 9]) {
    for stack in stacks.iter() {
        let last = match stack.last() {
            Some(last) => last,
            None => &' ',
        };
        print!("{}", last);
    }
    println!();
}

/// Parse move from the line "move x from y to z"
fn parse_move(line: &String) -> Move {
    let parts: Vec<&str> = line.split(' ').collect();
    let num: i32 = parts[1].parse().expect("Could not parse num");
    let source: usize = parts[3].parse().expect("Could not parse source");
    let dest: usize = parts[5].parse().expect("Could not parse dest");
    return Move {
        num,
        source,
        dest,
    };
}

/// Execute the move on the stacks
fn execute_move(m: &Move, stacks: &mut [Vec<char>; 9]) {
    for _ in 0..m.num {
        let c = stacks[m.source - 1].pop().expect("Could not pop from source");
        stacks[m.dest - 1].push(c);
    }
}

fn main() {
    let mut stacks = initialize_stacks();

    // read lines from the file one-by-one
    let file = File::open("data/day5.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line");

        let m = parse_move(&line);
        execute_move(&m, &mut stacks);
    }

    print_stack_tops(&stacks);
}
