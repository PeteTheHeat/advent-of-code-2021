use std::fs;

fn parse_input() -> String {
    fs::read_to_string("./../input.txt").unwrap()
}

fn main() {
    println!("Part 1: {}", solve_part_1(parse_input()));
    println!("Part 2: {}", solve_part_2(parse_input()));
}

fn solve_part_1(contents: String) -> i32 {
    todo!();
}

fn solve_part_2(contents: String) -> i32 {
    todo!();
}
