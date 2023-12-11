use std::fs::File;
use std::io::{prelude::*, BufReader};
use super::super::utils::parsers;


// Look up map
// Given a src number, we can calculate the dst number by:
// src_num + dst_src_diff
#[derive(Debug, Clone, Copy)]
struct Mapper {
  src_min: isize,
  src_max: isize,
  dst_min: isize,
  dst_max: isize,
  dst_src_diff: isize, // this should be calculated via dst_min - src_min,
  range_length: usize
}

#[allow(dead_code)]
pub fn run() {
  let file = File::open("./data/day5_test.txt").unwrap();
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
  for i in 0..=1000 {
    
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
  let dst_range_end: isize = dst_range_start + range_length - 1;
  let dst_src_diff: isize = dst_range_start - src_range_start;

  // create the mapper
  let mapper: Mapper = Mapper {
    src_min: src_range_start,
    src_max: src_range_end,
    dst_min: dst_range_start,
    dst_max: dst_range_end,
    dst_src_diff: dst_src_diff,
    range_length: range_length as usize
  };

  // add it to the vector
  maps.push(mapper);
}

// Goes from dst range and work backwards
fn get_src_from_dst(dst_num: isize, maps: &Vec<Mapper>) -> isize {
  for mapper in maps {
    if dst_num <= mapper.dst_max && dst_num >= mapper.dst_min {
      return dst_num - mapper.dst_src_diff
    }
  }
  // If not in a map, just return the source number
  return dst_num;
}

fn get_nth_lowest_location(location_maps: &Vec<Mapper>, n: isize) -> isize {
  let location_maps_sorted: Vec<Mapper> = sort_maps(location_maps);

  let mut n_clone = n.clone();
  let mut index = 0;

  while n_clone != 0 {
    let temp_nclone = n_clone - location_maps_sorted[index].range_length as isize;
    if temp_nclone > 0 {
      n_clone = temp_nclone;
      index += 1;
    } else {
      return location_maps_sorted[index].dst_min + (n_clone as isize) - 1;
    }
  }
  return std::isize::MAX;
}

// sort maps by destination number
// gonna make a really slow algorithm just cause this is a defined space
fn sort_maps(maps: &Vec<Mapper>) -> Vec<Mapper> {
  let mut compare_maps: Vec<Mapper> = maps.clone();
  let mut new_maps: Vec<Mapper> = Vec::new();
  while compare_maps.len() != 0 {
    let next_lowest_selected_map_index: usize = get_lowest_map(&compare_maps);
    let next_lowest_selected_map: Mapper = compare_maps[next_lowest_selected_map_index].clone();
    
    new_maps.push(next_lowest_selected_map);
    compare_maps.remove(next_lowest_selected_map_index);
  }

  return new_maps;
}

// get the index of the lowest map in maps
fn get_lowest_map(maps: &Vec<Mapper>) -> usize {
  // guard
  if maps.len() == 0 {
    panic!("Should not be given empty map");
  }

  let mut lowest_dst: isize = std::isize::MAX;
  for (index, mapper) in maps.iter().enumerate() {
    if mapper.dst_min < lowest_dst {
      return index;
    }
  }
  panic!("Should not get here!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day5part2() {
    println!("Running!");
    run();
  }

  #[test]
  fn test_get_nth_lowest_location() {
    let map1: Mapper = Mapper {
      src_min: 60,
      src_max: 60+37-1,
      dst_min: 56,
      dst_max: 56+37-1,
      dst_src_diff: 56-60,
      range_length: 37
    };
    let map2: Mapper = Mapper {
      src_min: 56,
      src_max: 56+4-1,
      dst_min: 93,
      dst_max: 93+4-1,
      dst_src_diff: 93-56,
      range_length: 4
    };
    let location_maps: Vec<Mapper> = vec![map1, map2];
    assert_eq!(56, get_nth_lowest_location(&location_maps, 1));
    assert_eq!(57, get_nth_lowest_location(&location_maps, 2));
    assert_eq!(92, get_nth_lowest_location(&location_maps, 37));
    assert_eq!(93, get_nth_lowest_location(&location_maps, 38));
  }
}
