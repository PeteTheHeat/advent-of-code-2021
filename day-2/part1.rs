use std::fs;

fn main() {
   let contents = fs::read_to_string("input.txt").unwrap();
   let mut horizontal = 0;
   let mut depth = 0;

    for line in contents.lines(){
        let vec:Vec<&str> = line.split(" ").collect();
        let word = vec[0];
        let value = vec[1].parse::<i32>().unwrap();

        if word == "forward" {
            horizontal += value;
        } else if word == "up" {
            depth -= value;
        } else if word == "down" {
            depth += value;
        } else {
            panic!();
        }
    }

    println!("Horizontal: {} Depth: {} Multiplied: {}", horizontal.to_string(), depth.to_string(), (horizontal*depth).to_string());

}
