use std::fs;

fn main() {
   let contents = fs::read_to_string("input.txt").unwrap();
   let mut answer_count = 0;
   let mut prev_value = i32::MAX;
   let contents_vec: Vec<i32> = contents.lines().map(|line| line.parse::<i32>().unwrap()).collect();

   for i in 0..contents_vec.len()-2 {
       println!("Value {}", contents_vec[i].to_string());
       let current_value = contents_vec[i] + contents_vec[i+1] + contents_vec[i+2];
       if current_value > prev_value {
           answer_count += 1;
       }
       prev_value = current_value;
   }
   println!("Answer is {}", answer_count.to_string());
}
