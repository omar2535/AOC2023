use std::fs::File;
use std::io::{prelude::*, BufReader};
use super::super::utils::parsers;
use super::super::utils::vec_utils;

struct Race {
  time: usize,
  distance: usize
}

#[allow(dead_code)]
pub fn run() {
  let file = File::open("./data/day6_input.txt").unwrap();
  let reader = BufReader::new(file);

  let mut times: Vec<usize> = Vec::new();
  let mut distances: Vec<usize> = Vec::new();

  for (index, line) in reader.lines().enumerate() {
    let cur_line: String = line.unwrap();

    if index == 0 {
      times = parsers::parse_numbers_after_substring(
        &cur_line,
        "Time:      ".to_string(),
        ' '
      )
    } else if index == 1 {
      distances = parsers::parse_numbers_after_substring(
        &cur_line,
        "Distance:".to_string(),
        ' '
      )
    } else {
      continue
    }
    println!("{}", cur_line);
  }

  // get the large numbers
  let time: usize = vec_utils::concat(&times);
  let distance: usize = vec_utils::concat(&distances);
  let race: Race = Race { time: time, distance: distance };
  let wins = get_num_wins(race);
  
  println!("{:?}", time);
  println!("{:?}", distance);
  println!("Wins {}", wins);
}

fn get_num_wins(race: Race) -> usize {
  let mut num_wins: usize = 0;
  for i in 0..=race.time {
    let hold_time: usize = i;
    let remaining_time: usize = race.time - hold_time;
    let speed: usize = hold_time;
    let distance_traveled = remaining_time * speed;
    
    if distance_traveled > race.distance {
      num_wins += 1;
    }
  }
  return num_wins;
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    run();
  }
}
