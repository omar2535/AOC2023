use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use super::super::utils::parsers;

#[allow(dead_code)]
pub fn run() {
  let file = File::open("./data/day5_test.txt").unwrap();
  let reader = BufReader::new(file);
  

  // variables to keep track of
  let mut seeds: Vec<usize> = Vec::new();
  let mut seed_to_soil_map: HashMap<usize, usize> = HashMap::new();
  let mut soil_to_fertilizer_map: HashMap<usize, usize> = HashMap::new();
  let mut fertilizer_to_water_map: HashMap<usize, usize> = HashMap::new();
  let mut water_to_light_map: HashMap<usize, usize> = HashMap::new();
  let mut light_to_temperature_map: HashMap<usize, usize> = HashMap::new();
  let mut temperature_to_humidity_map: HashMap<usize, usize> = HashMap::new();
  let mut humidity_to_location_map: HashMap<usize, usize> = HashMap::new();


  // first pass, read all the numbers and collect information about them
  let mut lines = reader.lines().peekable();
  let mut index = 0;
  let mut should_stop = false;
  let mut map_iter = 0;

  while !should_stop {
    // get the line
    let cur_line: String = match lines.next() {
      Some(line) => line.unwrap(),
      None => {should_stop = true; String::from(" ")},
    };
    println!("{}", cur_line);

    // If the line number is 1, parse the seeds
    if index == 0 {
      seeds = parsers::parse_numbers_after_substring(&cur_line, String::from("seeds: "), ' ');
    }

    // if line contains the word map, map each line until the next line is the word map into
    if cur_line.find(&"map").is_some() {
      // just update the map iter
      map_iter += 1;
    } else {
      // Update the maps here
      if cur_line == "" {
        // do nothing with current line if it was empty
      } else {
        match map_iter {
          1 => add_to_map(&cur_line, &mut seed_to_soil_map),
          2 => add_to_map(&cur_line, &mut soil_to_fertilizer_map),
          3 => add_to_map(&cur_line, &mut fertilizer_to_water_map),
          4 => add_to_map(&cur_line, &mut water_to_light_map),
          5 => add_to_map(&cur_line, &mut light_to_temperature_map),
          6 => add_to_map(&cur_line, &mut temperature_to_humidity_map),
          7 => add_to_map(&cur_line, &mut humidity_to_location_map),
          _ => panic!("Should not have an map iter that isn't in range! {}", map_iter),
        }
      }
    }

    // update the index
    index += 1;
  }
  println!("Done at index: {}!", index);
}

// adds to the map given by the reference
fn add_to_map(line: &String, map: &mut HashMap<usize, usize>) {
  // TODO
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day3part1() {
      run();
  }
}
