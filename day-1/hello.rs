use std::fs;

fn main() {
   let contents = fs::read_to_string("input.txt").unwrap();
   let mut answer_count = 0;
   let mut prev_value = i32::MAX;

   for line in contents.lines(){
       let current_value = line.parse::<i32>().unwrap();
       if current_value > prev_value {
           answer_count += 1;
       }
       prev_value = current_value;
   }
   println!("Answer is {}", answer_count.to_string());
}
