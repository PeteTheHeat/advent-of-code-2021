use std::fs;

fn main() {
   let contents = fs::read_to_string("input.txt").unwrap();
   let mut horizontal = 0;
   let mut depth = 0;
   let mut aim = 0;

    for line in contents.lines(){
        let vec:Vec<&str> = line.split(" ").collect();
        let word = vec[0];
        let value = vec[1].parse::<i32>().unwrap();

        if word == "forward" {
            horizontal += value;
            depth += value*aim;
        } else if word == "up" {
            aim -= value;
        } else if word == "down" {
            aim += value;
        } else {
            panic!();
        }
    }

    println!("Horizontal: {} Depth: {} Multiplied: {}", horizontal.to_string(), depth.to_string(), (horizontal*depth).to_string());

}
