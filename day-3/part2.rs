use std::fs;

fn main() {
   let contents = fs::read_to_string("input.txt").unwrap();
   let mut input:Vec<Vec<char>> = Vec::new();

   //Construct input
   for line in contents.lines(){
      let mut vec:Vec<char> = Vec::new();
      for bit in line.chars() {
         vec.push(bit);
      }
       input.push(vec);
    }

    let mut index = 0;

    loop {
      let mut count = 0;
      //Calculate most common
      for entry in &input{
         match entry[index] {
            '0' => count -=1,
            '1' => count +=1,
            _ => panic!(),
         }
      }

      //Filter most/least common
      if count < 0 {
         input.retain(|i| i[index] == '1');
      } else {
         input.retain(|i| i[index] == '0');
      }
      count = 0;
      index += 1;

      for entry in &input{
         println!("Done round {} option: {:?}", index.to_string(), entry);
      }

      if input.len() == 1 {
         println!("Done! Answer: {:?}", input[0]);
         return;
      }
    }
}
