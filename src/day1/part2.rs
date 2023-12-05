use std::fs::File;
use std::io::{prelude::*, BufReader};

#[allow(dead_code)]
pub fn parttwo() {
  let file = File::open("./data/day1.txt").unwrap();
  let reader = BufReader::new(file);

  let mut sum: i32 = 0;

  for line in reader.lines() {
    let cur_line: String = line.unwrap();
    let leftmost_num: char = find_leftmost_num(cur_line.clone());
    let rightmost_num: char = find_rightmost_num(cur_line.clone());
    let number_str: String = format!("{}{}", leftmost_num, rightmost_num);
    let number: i32 = number_str.parse().unwrap();
    sum += number;
    println!("Left: {}, Right: {}, sum: {}", leftmost_num, rightmost_num, sum);
  }
  println!("Sum: {}", sum);
}

fn find_leftmost_num(str: String) -> char {
  let mut seen_str: String = "".to_owned();

  for c in str.chars() {
    let seen_str_temp = format!("{}{}", seen_str, c);

    if c.is_numeric() {
      return c;
    } else if seen_str_temp.contains("one") {
      return '1';
    } else if seen_str_temp.contains("two") {
      return '2';
    } else if seen_str_temp.contains("three") {
      return '3';
    } else if seen_str_temp.contains("four") {
      return '4';
    } else if seen_str_temp.contains("five") {
      return '5';
    } else if seen_str_temp.contains("six") {
      return '6';
    } else if seen_str_temp.contains("seven") {
      return '7';
    } else if seen_str_temp.contains("eight") {
      return '8';
    } else if seen_str_temp.contains("nine") {
      return '9';
    } else {
      seen_str = seen_str_temp;
    }
  }

  return '0';
}

fn find_rightmost_num(str: String) -> char {
  let mut seen_str: String = "".to_owned();

  for c in str.chars().rev() {
    let seen_str_temp = format!("{}{}", c, seen_str);

    if c.is_numeric() {
      return c;
    } else if seen_str_temp.contains("one") {
      return '1';
    } else if seen_str_temp.contains("two") {
      return '2';
    } else if seen_str_temp.contains("three") {
      return '3';
    } else if seen_str_temp.contains("four") {
      return '4';
    } else if seen_str_temp.contains("five") {
      return '5';
    } else if seen_str_temp.contains("six") {
      return '6';
    } else if seen_str_temp.contains("seven") {
      return '7';
    } else if seen_str_temp.contains("eight") {
      return '8';
    } else if seen_str_temp.contains("nine") {
      return '9';
    } else {
      seen_str = seen_str_temp;
    }
  }

  return '0';
}
