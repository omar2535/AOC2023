use std::fs::File;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};

pub fn p1() {
  let file = File::open("./data/day2.txt").unwrap();
  let reader = BufReader::new(file);

  let mut sum: i32 = 0;

  for line in reader.lines() {
    let cur_line: String = line.unwrap();
    let mut cube_counts: HashMap<String, i32> = initialize_empty_cube_counts();
    

    println!("{}", cur_line);
  }
}

fn initialize_empty_cube_counts() -> HashMap<String, i32> {
  let cube_counts: HashMap<String, i32> = HashMap::from([
      (String::from("blue"), 0),
      (String::from("red"), 0),
      (String::from("green"), 0)
    ]);
  return cube_counts;
}
