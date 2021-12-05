use std::fs;

fn main() {
   let contents = fs::read_to_string("input.txt").unwrap();
   let mut data:Vec<i32> = [0,0,0,0,0,0,0,0,0,0,0,0].to_vec();
   let mut num_lines = 0;

   for line in contents.lines(){
       num_lines +=1;
       let vec:Vec<char> = line.chars().collect();
       for i in 0..12 {
            if vec[i] == '1' {
                data[i] += 1;
            }
       }
    }

    for i in 0..12 {
        if num_lines-data[i] > data[i] {
            println!("Position {} is 0", i.to_string());
        } else {
            println!("Position {} is 1", i.to_string());
        }
    }

}
