use std::fs::File;
use std::clone::Clone;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::{prelude::*, BufReader};

#[derive(Clone, PartialEq, Eq, Hash)]
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

impl PartialEq for Coordinate {
  fn eq(&self, other: &Self) -> bool {
    return self.x == other.x && self.y == other.y;
  }
}

impl Hash for Coordinate {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.x.hash(state);
    self.y.hash(state);
  }
}

impl Eq for Coordinate {}

#[allow(dead_code)]
pub fn run() {
  let file = File::open("./data/day3_input.txt").unwrap();
  let reader = BufReader::new(file);

  let mut numbers_vec: Vec<Number> = vec![];
  let mut grid: Vec<Vec<char>> = Vec::new();
  let mut number_mapper: HashMap<Coordinate, Number> = HashMap::new();
  let mut sum: u32 = 0;

  // first pass, read all the numbers and collect information about them
  for (y, line) in reader.lines().enumerate() {
    // get the line
    let cur_line: String = line.unwrap();
    // println!("{}", cur_line);
    
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

        // push the number, add the number to the coordinate-number map, and clear the built string number
        for new_number_x in new_number.start_index..=new_number.end_index {
          let coordinate: Coordinate = Coordinate { x: new_number_x, y: y };
          number_mapper.insert(coordinate, new_number.clone());
        }
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
  // println!("Max X: {}, Max Y: {}", max_x, max_y);
  for y in 0..max_y {
    for x in 0..max_x {
      if grid[y][x] == '*' {
        let coordinate = Coordinate { x: x, y: y };
        sum += get_gear_multiple(coordinate, &number_mapper, max_x, max_y)
      }
    }
  }

  println!("Sum: {}", sum);
}


// Check cordinates and get the gear multiple, 0 if no adjacent numbers or more than 2 adjacent numbers
fn get_gear_multiple(coordinate: Coordinate, number_mapper: &HashMap<Coordinate, Number>,  max_x: usize, max_y: usize) -> u32 {
  // make sure we only saw 2 numbers!
  let mut unique_number_tracker: HashMap<Number, bool> = HashMap::new();

  // Check the ones at the bottom
  if coordinate.y != max_y - 1 {
    let search_coordinate: Coordinate = Coordinate { x: coordinate.x, y: coordinate.y + 1 };
    if number_mapper.contains_key(&search_coordinate) && !unique_number_tracker.contains_key(&number_mapper.get(&search_coordinate).unwrap().clone()) {
      unique_number_tracker.insert(number_mapper.get(&search_coordinate).unwrap().clone(), true);
    }
  }

  // add the ones at the top
  if coordinate.y != 0 {
    let search_coordinate: Coordinate = Coordinate { x: coordinate.x, y: coordinate.y - 1 };
    if number_mapper.contains_key(&search_coordinate) && !unique_number_tracker.contains_key(&number_mapper.get(&search_coordinate).unwrap().clone()) {
      unique_number_tracker.insert(number_mapper.get(&search_coordinate).unwrap().clone(), true);
    }
  }

  // add the ones on the right
  if coordinate.x != max_x - 1 {
    let search_coordinate: Coordinate = Coordinate { x: coordinate.x+1, y: coordinate.y };
    if number_mapper.contains_key(&search_coordinate) && !unique_number_tracker.contains_key(&number_mapper.get(&search_coordinate).unwrap().clone()) {
      unique_number_tracker.insert(number_mapper.get(&search_coordinate).unwrap().clone(), true);
    }
  }

  // add the ones on the left
  if coordinate.x != 0 {
    let search_coordinate: Coordinate = Coordinate { x: coordinate.x-1, y: coordinate.y };
    if number_mapper.contains_key(&search_coordinate) && !unique_number_tracker.contains_key(&number_mapper.get(&search_coordinate).unwrap().clone()) {
      unique_number_tracker.insert(number_mapper.get(&search_coordinate).unwrap().clone(), true);
    }
  }

  // add top right if not at edge
  if coordinate.x != max_x - 1 && coordinate.y != 0 {
    let search_coordinate: Coordinate = Coordinate { x: coordinate.x+1, y: coordinate.y-1 };
    if number_mapper.contains_key(&search_coordinate) && !unique_number_tracker.contains_key(&number_mapper.get(&search_coordinate).unwrap().clone()) {
      unique_number_tracker.insert(number_mapper.get(&search_coordinate).unwrap().clone(), true);
    }
  }

  // add top left if not at edge
  if coordinate.x != 0 && coordinate.y != 0 {
    let search_coordinate: Coordinate = Coordinate { x: coordinate.x-1, y: coordinate.y-1 };
    if number_mapper.contains_key(&search_coordinate) && !unique_number_tracker.contains_key(&number_mapper.get(&search_coordinate).unwrap().clone()) {
      unique_number_tracker.insert(number_mapper.get(&search_coordinate).unwrap().clone(), true);
    }
  }

  // add bottom left if not at edge
  if coordinate.x != 0 && coordinate.y != max_y - 1 {
    let search_coordinate: Coordinate = Coordinate { x: coordinate.x-1, y: coordinate.y+1 };
    if number_mapper.contains_key(&search_coordinate) && !unique_number_tracker.contains_key(&number_mapper.get(&search_coordinate).unwrap().clone()) {
      unique_number_tracker.insert(number_mapper.get(&search_coordinate).unwrap().clone(), true);
    }
  }

  // add bottom right if not at edge
  if coordinate.x != max_x - 1 && coordinate.y != max_y - 1 {
    let search_coordinate: Coordinate = Coordinate { x: coordinate.x+1, y: coordinate.y+1 };
    if number_mapper.contains_key(&search_coordinate) && !unique_number_tracker.contains_key(&number_mapper.get(&search_coordinate).unwrap().clone()) {
      unique_number_tracker.insert(number_mapper.get(&search_coordinate).unwrap().clone(), true);
    }
  }

  // if there's exactly only 2 keys, then return the multiplication of them. Otherwise, return 0
  let mut multiple: u32 = 0;
  if unique_number_tracker.keys().count() == 2 {
    multiple = 1;
    for key in unique_number_tracker.keys() {
      multiple = multiple * key.number;
    }
  }

  return multiple;
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day3part2() {
      run();
  }
}
