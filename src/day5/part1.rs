use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use super::super::utils::parsers;

#[allow(dead_code)]
pub fn run() {
  let file = File::open("./data/day5_input.txt").unwrap();
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
      index += 1;
      continue;
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

  let mut min_location = std::usize::MAX;
  for seed_num in &seeds {
    let location_num = convert_seed_to_location(
      *seed_num,
      &seed_to_soil_map,
      &soil_to_fertilizer_map,
      &fertilizer_to_water_map,
      &water_to_light_map,
      &light_to_temperature_map,
      &temperature_to_humidity_map,
      &humidity_to_location_map
    );

    if location_num <= min_location {
      min_location = location_num;
    }
  }

  println!("Seeds: {:?}", seeds);
  println!("Minimum location number: {}", min_location);
  println!("Done at index: {}!", index);
}

// adds to the map given by the reference
fn add_to_map(line: &String, map: &mut HashMap<usize, usize>) {
  let numbers: Vec<usize> = parsers::parse_numbers_after_substring(line, String::from(""), ' ');
  if numbers.len() == 0 {
    return;
  }

  let dst_range_start: usize = numbers[0];
  let src_range_start: usize = numbers[1];
  let range_length: usize = numbers[2];

  for index in 0..range_length {
    if map.contains_key(&(src_range_start + index)) {
      panic!("Should not have a duplicate key! {}", src_range_start + index);
    }
    map.insert(src_range_start + index, dst_range_start + index);
  }
}

// get the seed from a location
fn convert_seed_to_location(seed_num: usize,
                            seed_to_soil_map: &HashMap<usize, usize>,
                            soil_to_fertilizer_map: &HashMap<usize, usize>,
                            fertilizer_to_water_map: &HashMap<usize, usize>,
                            water_to_light_map: &HashMap<usize, usize>,
                            light_to_temperature_map: &HashMap<usize, usize>,
                            temperature_to_humidity_map: &HashMap<usize, usize>,
                            humidity_to_location_map: &HashMap<usize, usize>) -> usize {
  println!("Seed number: {}", seed_num);
  let soil_num: usize = match seed_to_soil_map.get(&seed_num) {
    Some(soil_num) => *soil_num,
    None => seed_num
  };
  let fertilizer_num: usize = match soil_to_fertilizer_map.get(&soil_num) {
    Some(fertilizer_num) => *fertilizer_num,
    None => soil_num
  };
  let water_num: usize = match fertilizer_to_water_map.get(&fertilizer_num) {
    Some(water_num) => *water_num,
    None => fertilizer_num
  };
  let light_num: usize = match water_to_light_map.get(&water_num) {
    Some(light_num) => *light_num,
    None => water_num
  };
  let temperature_num: usize = match light_to_temperature_map.get(&light_num) {
    Some(temperature_num) => *temperature_num,
    None => light_num
  };
  let humidity_num: usize = match temperature_to_humidity_map.get(&temperature_num) {
    Some(humidity_num) => *humidity_num,
    None => temperature_num
  };
  let location_num: usize = match humidity_to_location_map.get(&humidity_num) {
    Some(location_num) => *location_num,
    None => humidity_num
  };
  return location_num;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day5part1r() {
    println!("Running!");
    run();
  }
}
 