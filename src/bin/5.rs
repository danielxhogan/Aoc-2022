use core::str::Lines;
use std::collections::{HashMap, VecDeque};
use std::cell::RefCell;

type MoveInstruction = (u32, u32, u32);

fn organize_data(lines: &mut Lines<'_>) -> HashMap<u32, RefCell<VecDeque<char>>> {
    let mut cols: HashMap<u32, RefCell<VecDeque<char>>> = HashMap::new();

    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }

        let mut chars = line.chars();
        chars.next();
        let mut i = 1;

        while let Some(c) = chars.next() {
            if c != ' ' && c.to_string() != i.to_string() {
                let maybe_col = cols.get(&i);

                match maybe_col {
                    Some(col) => col.borrow_mut().push_front(c),
                    None => {
                        let new_col = RefCell::new(VecDeque::new());
                        new_col.borrow_mut().push_front(c);
                        cols.insert(i, new_col);
                    }
                }
            }

            chars.next();
            chars.next();
            chars.next();
            i += 1;
        }
    }

    cols
}

// part 1
fn move_containers(line: &str, cols: &mut HashMap<u32, RefCell<VecDeque<char>>>) {
    let instructions = parse_instructions(line).unwrap();
    let mut col1 = cols.get(&instructions.1).expect("column moving from exists").borrow_mut();
    let mut col2 = cols.get(&instructions.2).expect("column moving to exitsts").borrow_mut();
    let mut moving_container: char;
    
    for _ in 0..instructions.0 {
        moving_container = col1.pop_back().expect("the column removing has containers to remove");
        col2.push_back(moving_container);
    }
}

// part 2
fn move_containers2(line: &str, cols: &mut HashMap<u32, RefCell<VecDeque<char>>>) {
    let instructions = parse_instructions(line).unwrap();
    let mut col1 = cols.get(&instructions.1).expect("column moving from exists").borrow_mut();
    let mut col2 = cols.get(&instructions.2).expect("column moving to exitsts").borrow_mut();

    let mut moving_containers: Vec<char> = Vec::new();
    let mut moving_container: char;
    
    for _ in 0..instructions.0 {
        moving_container = col1.pop_back().expect("the column removing has containers to remove");
        moving_containers.push(moving_container);
    }

    for _ in 0..instructions.0 {
        col2.push_back(moving_containers.pop().unwrap());
    }
}

fn parse_instructions(instruction: &str) -> Result<MoveInstruction, &str> {
    let parts: Vec<&str> = instruction.split(" ").collect();

    if parts.len() != 6 {
        return Err("not a valid instruction");
    }

    let mut parts = parts.iter();
    let mut instructions: Vec<u32> = Vec::new();

    while let Some(_) = parts.next() {
        let instruction = parts
            .next()
            .expect("to get an instruction")
            .parse::<u32>()
            .expect("instruction is a number");

        instructions.push(instruction);
    }

    Ok((instructions[0], instructions[1], instructions[2]))
}

fn main() {
    let mut lines = include_str!("../input/5.txt").lines();
    let mut cols = organize_data(&mut lines);

    // part 1
    // while let Some(line) = lines.next() {
    //     move_containers(line, &mut cols);
    // }

    // part 2
    while let Some(line) = lines.next() {
        move_containers2(line, &mut cols);
    }

    for i in 1..=cols.len() {
        let x = i as u32;
        let col = cols.get(&x).unwrap().borrow_mut();
        println!("{}", col[col.len()-1]);

    }
}
