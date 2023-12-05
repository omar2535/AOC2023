use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Number {
  number: u32,
  line_number: usize,
  start_index: usize,
  end_index: usize,
}

struct Coordinate {
  x: usize,
  y: usize
}

#[allow(dead_code)]
pub fn run() {
  let file = File::open("./data/day3_input.txt").unwrap();
  let reader = BufReader::new(file);

  let mut numbers_vec: Vec<Number> = vec![];
  let mut grid: Vec<Vec<char>> = Vec::new();
  let mut sum: u32 = 0;

  // first pass, read all the numbers and collect information about them
  for (y, line) in reader.lines().enumerate() {
    // get the line
    let cur_line: String = line.unwrap();
    println!("{}", cur_line);
    
    // set some accumulators
    let mut vec: Vec<char> = Vec::new();
    let mut built_number: String = "".to_owned();
    let mut number: u32;
    for (x, c) in cur_line.chars().into_iter().enumerate() {
      // println!("x: {}, y: {}, C: {}", x, y, c);
      vec.push(c);

      // if it's a number, start accumulating
      if c.is_numeric() {
        built_number.push(c);
      }
      // if it's the last position or the next character isn't a number, and there is something in the built number, create a number for it
      if (x + 1  == cur_line.chars().count() || !cur_line.chars().nth(x + 1).unwrap().is_numeric()) && (built_number != "") {
        number = built_number.parse().unwrap();
        let new_number: Number = Number {
          number: number,
          line_number: y,
          start_index: x - (built_number.chars().count() - 1),
          end_index: x
        };
        // push the number and clear
        numbers_vec.push(new_number);
        built_number = "".to_owned();
      }
    }
    // Add the vector to the grid
    grid.push(vec);
  }

  // Second pass: Check adjacent for any symbols
  // cases:
  // if at the top
  // if at the bottom
  // if at the most left edge
  // if at the most right edge
  let max_x: usize = grid.iter().nth(0).unwrap().len();
  let max_y: usize = grid.len();
  println!("Max X: {}, Max Y: {}", max_x, max_y);
  for number in numbers_vec.iter() {
    let coordinates_to_check = get_all_cordinates_to_check(number, max_x, max_y);
    if has_adjacent_symbols(coordinates_to_check, &grid) {
      println!("Number: {} has adjacent symbols!", number.number);
      sum += number.number;
    }
  }

  println!("Sum: {}", sum);
}

fn has_adjacent_symbols(coordinates_to_check: Vec<Coordinate>, grid: &Vec<Vec<char>>) -> bool {
  for coordinate in coordinates_to_check.iter() {
    if grid[coordinate.y][coordinate.x] != '.' && !grid[coordinate.y][coordinate.x].is_alphanumeric() {
      return true;
    }
  }
  return false;
}

// get all cordinates to check
fn get_all_cordinates_to_check(number: &Number, max_x: usize, max_y: usize) -> Vec<Coordinate> {
  let mut coordinates_to_check: Vec<Coordinate> = Vec::new();
  // add the ones at the bottom
  if number.line_number != max_y - 1 {
    for x in number.start_index..=number.end_index {
      let coordinate: Coordinate = Coordinate { x: x, y: number.line_number + 1 };
      coordinates_to_check.push(coordinate);
    }
  }

  // add the ones at the top
  if number.line_number != 0 {
    for x in number.start_index..=number.end_index {
      let coordinate: Coordinate = Coordinate { x: x, y: number.line_number - 1 };
      coordinates_to_check.push(coordinate);
    }
  }

  // add the ones on the right
  if number.end_index != max_x - 1 {
    let coordinate: Coordinate = Coordinate { x: number.end_index + 1, y: number.line_number };
    coordinates_to_check.push(coordinate);
  }

  // add the ones on the left
  if number.start_index != 0 {
    let coordinate: Coordinate = Coordinate { x: number.start_index - 1, y: number.line_number };
    coordinates_to_check.push(coordinate);
  }

  // add top right if not at edge
  if number.end_index != max_x - 1 && number.line_number != 0 {
    let coordinate: Coordinate = Coordinate { x: number.end_index + 1, y: number.line_number - 1 };
    coordinates_to_check.push(coordinate);
  }

  // add top left if not at edge
  if number.start_index != 0 && number.line_number != 0 {
    let coordinate: Coordinate = Coordinate { x: number.start_index - 1, y: number.line_number - 1 };
    coordinates_to_check.push(coordinate);
  }

  // add bottom left if not at edge
  if number.start_index != 0 && number.line_number != max_y - 1 {
    let coordinate: Coordinate = Coordinate { x: number.start_index - 1, y: number.line_number + 1 };
    coordinates_to_check.push(coordinate);
  }

  // add bottom right if not at edge
  if number.end_index != max_x - 1 && number.line_number != max_y - 1 {
    let coordinate: Coordinate = Coordinate { x: number.end_index + 1, y: number.line_number + 1 };
    coordinates_to_check.push(coordinate);
  }

  return coordinates_to_check;
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day3part1() {
      run();
  }
}
