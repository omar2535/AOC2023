use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[allow(dead_code)]
pub fn run() {
  let file = File::open("./data/day4_input.txt").unwrap();
  let reader = BufReader::new(file);
  let mut sum: i32 = 0;

  // first pass, read all the numbers and collect information about them
  for line in reader.lines() {
    // get the line
    let cur_line: String = line.unwrap();
    println!("{}", cur_line);
    
    // split the winning numbers and the have numbers
    let mut split_card_number = cur_line.split(':');
    let numbers: &str = split_card_number.nth(1).unwrap();
    let number_sets = numbers.split('|');

    // parse each set
    let mut map: HashMap<u32, u32> = HashMap::new();
    for (index, number_set) in number_sets.enumerate() {
      let number_set_vector = parse_into_array_of_u32(number_set);
      println!("{}, {:?}", index, number_set_vector);

      // this is the winning set map
      if index == 0 {
        map = parse_vec_into_map(number_set_vector);
      } else if index == 1 {
        let winning_count = count_num_winning_cards(number_set_vector, map.clone());
        if winning_count != 0 {
          let two: i32 = 2;
          let score = two.pow(winning_count - 1);
          sum += score;
        }
      }
    }
  }

  println!("Sum: {}", sum);
}

fn parse_into_array_of_u32(number_set: &str) -> Vec<u32> {
  let numbers = number_set.split(' ');
  let mut number_set_vec: Vec<u32> = Vec::new();
  for number in numbers {
    match number.parse::<u32>() {
      Ok(n) => {
        number_set_vec.push(n);
      },
      Err(_e) => (),
    };
  }
  return number_set_vec;
}

fn parse_vec_into_map(number_set_vector: Vec<u32>) -> HashMap<u32, u32> {
  let mut map: HashMap<u32, u32> = HashMap::new();
  for number in number_set_vector {
    if map.contains_key(&number) {
      *map.get_mut(&number).unwrap() += 1
    } else {
      map.insert(number, 1);
    }
  }
  return map;
}

fn count_num_winning_cards(number_set_vector: Vec<u32>, map: HashMap<u32, u32>) -> u32 {
  let mut count: u32 = 0;
  for number in number_set_vector {
    if map.contains_key(&number) {
      count += 1;
    }
  }
  return count;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day3part1() {
      run();
  }
}
