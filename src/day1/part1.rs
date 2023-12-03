use std::fs;

#[allow(dead_code)]
pub fn partone() {
  let data = fs::read_to_string("./data/day1a.txt").expect("Unable to read file");
  let mut sum: i32 = 0;

  let mut line_numbers: String = "".to_owned();
  for c in data.chars() {
    // println!("{}", c);

    // keep adding to line numbers as long as we are on the same line
    if c.is_numeric() {
      line_numbers.push(c);
    } else if c == '\n' {
      // now that we are on a new line, combine the first and last numbers and add to the sum
      let first_number: char = line_numbers.chars().nth(0).unwrap();
      let last_number: char = line_numbers.chars().last().unwrap();
      let combined_number_str: String = format!("{}{}", first_number, last_number);
      let combined_number: i32 = combined_number_str.parse().unwrap();
      // println!("sum: {}, combined_number: {}", sum, combined_number);

      // update the sum and reset the line
      sum += combined_number;
      line_numbers = "".to_owned();
    }

      // do something with `c`
  }

  println!("Total sum: {}", sum)
}
