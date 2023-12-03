use std::fs::File;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};

pub fn p1() {
  let file = File::open("./data/day2.txt").unwrap();
  let reader = BufReader::new(file);

  let mut sum: u32 = 0;

  for line in reader.lines() {
    let cur_line: String = line.unwrap();
    println!("{}", cur_line);

    // grab the game number
    let game_line = cur_line.split(':').next().unwrap();
    let game_number: u32 = parse_val(game_line);
    println!("Game number: {}", game_number);

    // get each set
    let games_str: &str = cur_line.split(':').nth(1).unwrap();
    let games = games_str.split(';');
    let mut is_possible_set: bool = true;

    // parse each game in the set
    for game in games {
      println!("game: {}", game);

      let game_cube_counts: HashMap<String, u32> = parse_game(game);
      if !is_possible_cube_count(game_cube_counts) {
        is_possible_set = false;
      }
    }

    // update the game nubmer if the set was possible
    if is_possible_set {
      println!("Game: {} is posibble!", game_number);
      sum += game_number;
    }
  }

  println!("Sum: {}", sum);
}

fn initialize_empty_cube_counts() -> HashMap<String, u32> {
  let cube_counts: HashMap<String, u32> = HashMap::from([
      (String::from("blue"), 0),
      (String::from("red"), 0),
      (String::from("green"), 0)
    ]);
  return cube_counts;
}

fn parse_game(game: &str) -> HashMap<String, u32> {
  let mut game_cube_counts: HashMap<String, u32> = initialize_empty_cube_counts();
  let cubes = game.split(',');

  for cube in cubes {
    println!("Cube: {}", cube);
    let val: u32 = parse_val(cube);
    if cube.find("red") != None {
      *game_cube_counts.get_mut("red").unwrap() += val;
    } else if cube.find("blue") != None {
      *game_cube_counts.get_mut("blue").unwrap() += val;
    } else if cube.find("green") != None {
      *game_cube_counts.get_mut("green").unwrap() += val;
    } else {
      println!("Couldn't find a proper color to add to for: {}", cube);
    }
  }
  return game_cube_counts;
}

// parses value from a string
fn parse_val(input: &str) -> u32 {
  let mut built_num: String = "".to_owned();
  for c in input.chars() {
    if c.is_numeric() {
      built_num = format!("{}{}", built_num, c);
    }
  }
  let val: u32 = built_num.parse().unwrap();
  return val;
}

// Checks if the cube count is possible given our restrictions
fn is_possible_cube_count(game_cube_counts: HashMap<String, u32>) -> bool {
  let red_count: u32 = *game_cube_counts.get("red").unwrap();
  let green_count: u32 = *game_cube_counts.get("green").unwrap();
  let blue_count: u32 = *game_cube_counts.get("blue").unwrap();
  println!("R: {}, G: {}, B: {}", red_count, green_count, blue_count);

  if red_count > 12 || green_count > 13 || blue_count > 14 {
    return false;
  } else {
    return true;
  }
}
