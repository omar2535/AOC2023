use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use super::super::utils::parsers;

#[allow(dead_code)]
pub fn run() {
  let file = File::open("./data/day5_test.txt").unwrap();
  let reader = BufReader::new(file);
  

  // variables to keep track of
  let seeds: Vec<usize> = Vec::new();
  let seed_to_soil_map: HashMap<usize, usize> = HashMap::new();
  let soil_to_fertilizer_map: HashMap<usize, usize> = HashMap::new();
  let fertilizer_to_water_map: HashMap<usize, usize> = HashMap::new();
  let water_to_light_map: HashMap<usize, usize> = HashMap::new();
  let light_to_temperature_map: HashMap<usize, usize> = HashMap::new();
  let temperature_to_humidity_map: HashMap<usize, usize> = HashMap::new();
  let humidity_to_location_map: HashMap<usize, usize> = HashMap::new();


  // first pass, read all the numbers and collect information about them
  for (index, line) in reader.lines().enumerate() {
    // get the line
    let cur_line: String = line.unwrap();
    println!("{}", cur_line);

    // If the line number is 1, parse the seeds
    if index == 0 {
      parsers::parse_numbers_after_substring(cur_line, String::from("seeds: "), ' ');
    }
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
