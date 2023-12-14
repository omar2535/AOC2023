use std::fs::File;
use std::io::{prelude::*, BufReader};


#[allow(dead_code)]
pub fn run() {
  let file = File::open("./data/day6_test.txt").unwrap();
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let cur_line: String = line.unwrap();
    println!("{}", cur_line);
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    run();
  }
}
