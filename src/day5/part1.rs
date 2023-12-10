use std::fs::File;
use std::io::{prelude::*, BufReader};
use super::super::utils::parsers;


// Look up map
// Given a src number, we can calculate the dst number by:
// src_num + dst_src_diff
struct Mapper {
  src_min: isize,
  src_max: isize,
  dst_src_diff: isize // this should be calculated via dst_min - src_min
}

#[allow(dead_code)]
pub fn run() {
  let file = File::open("./data/day5_input.txt").unwrap();
  let reader = BufReader::new(file);
  

  // variables to keep track of
  let mut seeds: Vec<isize> = Vec::new();
  let mut seed_to_soil_maps: Vec<Mapper> = Vec::new();
  let mut soil_to_fertizlier_maps: Vec<Mapper> = Vec::new();
  let mut fertilizer_to_water_maps: Vec<Mapper> = Vec::new();
  let mut water_to_light_maps: Vec<Mapper> = Vec::new();
  let mut light_to_temperature_maps: Vec<Mapper> = Vec::new();
  let mut temperature_to_humidity_maps: Vec<Mapper> = Vec::new();
  let mut humidity_to_location_maps: Vec<Mapper> = Vec::new();

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
      seeds = parsers::parse_numbers_after_substring_isize(&cur_line, String::from("seeds: "), ' ');
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
          1 => parse_into_map_vec(&cur_line, &mut seed_to_soil_maps),
          2 => parse_into_map_vec(&cur_line, &mut soil_to_fertizlier_maps),
          3 => parse_into_map_vec(&cur_line, &mut fertilizer_to_water_maps),
          4 => parse_into_map_vec(&cur_line, &mut water_to_light_maps),
          5 => parse_into_map_vec(&cur_line, &mut light_to_temperature_maps),
          6 => parse_into_map_vec(&cur_line, &mut temperature_to_humidity_maps),
          7 => parse_into_map_vec(&cur_line, &mut humidity_to_location_maps),
          _ => panic!("Should not have an map iter that isn't in range! {}", map_iter),
        }
      }
    }

    // update the index
    index += 1;
  }

  let mut min_location = std::isize::MAX;
  for seed_num in &seeds {
    let location_num = convert_seed_to_location(
      *seed_num,
      &seed_to_soil_maps,
      &soil_to_fertizlier_maps,
      &fertilizer_to_water_maps,
      &water_to_light_maps,
      &light_to_temperature_maps,
      &temperature_to_humidity_maps,
      &humidity_to_location_maps
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
fn parse_into_map_vec(line: &String, maps: &mut Vec<Mapper>) {
  let numbers: Vec<isize> = parsers::parse_numbers_after_substring_isize(line, String::from(""), ' ');
  if numbers.len() == 0 {
    return;
  }

  let dst_range_start: isize = numbers[0];
  let src_range_start: isize = numbers[1];
  let range_length: isize = numbers[2];

  // calculate some other numbers
  let src_range_end: isize = src_range_start + range_length - 1;
  let dst_src_diff: isize = dst_range_start - src_range_start;

  // create the mapper
  let mapper: Mapper = Mapper {
    src_min: src_range_start,
    src_max: src_range_end,
    dst_src_diff: dst_src_diff
  };

  // add it to the vector
  maps.push(mapper);
}

// searches a map, gets the next number
fn get_mapped_number(src_num: isize, maps: &Vec<Mapper>) -> isize {
  for mapper in maps {
    if src_num <= mapper.src_max && src_num >= mapper.src_min {
      return src_num + mapper.dst_src_diff
    }
  }
  // If not in a map, just return the source number
  return src_num;
}

// get the seed from a location
fn convert_seed_to_location(seed_num: isize,
                            seed_to_soil_maps: &Vec<Mapper>,
                            soil_to_fertilizer_maps: &Vec<Mapper>,
                            fertilizer_to_water_maps: &Vec<Mapper>,
                            water_to_light_maps: &Vec<Mapper>,
                            light_to_temperature_maps: &Vec<Mapper>,
                            temperature_to_humidity_maps: &Vec<Mapper>,
                            humidity_to_location_maps: &Vec<Mapper>) -> isize {
  println!("Seed number: {}", seed_num);
  let soil_num: isize = get_mapped_number(seed_num, seed_to_soil_maps);
  let fertilizer_num: isize = get_mapped_number(soil_num, soil_to_fertilizer_maps);
  let water_num: isize = get_mapped_number(fertilizer_num, fertilizer_to_water_maps);
  let light_num: isize = get_mapped_number(water_num, water_to_light_maps);
  let temperature_num: isize = get_mapped_number(light_num, light_to_temperature_maps);
  let humidity_num: isize = get_mapped_number(temperature_num, temperature_to_humidity_maps);
  let location_num: isize = get_mapped_number(humidity_num, humidity_to_location_maps);

  return location_num;
}


// I want a function that calculates the mapping of a seec to soil dynamically
// Create a new map of all the ranges, evaluate if the number is in the range, return the new number




#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day5part1() {
    println!("Running!");
    run();
  }
}
