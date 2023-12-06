use std::fs::File;
use std::io::{prelude::*, BufReader};

#[allow(dead_code)]
pub fn run() {
  let file = File::open("./data/day4_test.txt").unwrap();
  let reader = BufReader::new(file);

  // first pass, read all the numbers and collect information about them
  for line in reader.lines() {
    // get the line
    let cur_line: String = line.unwrap();
    println!("{}", cur_line);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day3part1() {
      run();
  }
}
