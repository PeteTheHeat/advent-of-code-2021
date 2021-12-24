use std::fs;

fn parse_input() -> String {
    fs::read_to_string("./../sample.txt").unwrap()
}

fn main() {
    println!("Part 1: {}", solve_part_1(parse_input()));
    println!("Part 2: {}", solve_part_2(parse_input()));
}

fn solve_part_1(contents: String) -> usize {
    let mut fish = parse_into_vec(contents);

    for _days in 0..80 {
        let new_fish_count = increment_fish(&mut fish);
        for _new_fish in 0..new_fish_count {
            fish.push(8);
        }
    }
    fish.len()
}

fn solve_part_2(_contents: String) -> i64 {
    recurse(4) // this doesn't finish at 260...
}

fn recurse(days_left: i32) -> i64 {
    let mut total = 1;

    let spawns = days_left / 7;
    // println!("Spawns: {:?}", spawns);
    for i in 1..spawns + 1 {
        total += recurse(days_left - 7 * i);
    }
    total
}

fn increment_fish(fish: &mut Vec<i32>) -> i32 {
    let mut new_fish_count = 0;
    for i in 0..fish.len() {
        if fish[i] == 0 {
            fish[i] = 6;
            new_fish_count += 1;
        } else {
            fish[i] = fish[i] - 1;
        }
    }
    new_fish_count
}

fn parse_into_vec(contents: String) -> Vec<i32> {
    contents
        .split(",")
        .map(|i| i.trim())
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}
