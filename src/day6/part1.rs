use std::fs::File;
use std::io::{prelude::*, BufReader};
use super::super::utils::parsers;

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

  // make sure the 2 lengths are equal
  assert_eq!(times.len(), distances.len());

  // create a vector of races
  let mut races: Vec<Race> = Vec::new();
  for i in 0..times.len() {
    let race: Race = Race { time: times[i], distance: distances[i] };
    races.push(race);
  }


  // for each race, determine number of ways we can beat it
  let mut multiple: usize = 1;
  for race in races {
    multiple = multiple * get_num_wins(race);
  }
  
  println!("{:?}", times);
  println!("{:?}", distances);
  println!("Multiple {}", multiple);
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
