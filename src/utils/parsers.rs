// parser utilities

// Parses numbers after a substring
pub fn parse_numbers_after_substring(input_string: String, substring: String, numbers_delim: char) -> Vec<usize> {
  let search_start_index: usize = input_string.find(&substring).unwrap() + substring.len(); 
  let string_search_space: String = input_string.chars().skip(search_start_index).take(input_string.len() - substring.len()).collect();

  let numbers = string_search_space.split(numbers_delim);
  let mut number_set_vec: Vec<usize> = Vec::new();
  for number in numbers {
    match number.parse::<usize>() {
      Ok(n) => {
        number_set_vec.push(n);
      },
      Err(e) => (),
    };
  }
  return number_set_vec;
}

#[cfg(test)]
mod tests {
    use super::parse_numbers_after_substring;

  #[test]
  fn test_parse_numbers_after_substring() {
    let input_string: String = String::from("seeds: 79 14 55 13");
    let substring: String = String::from("seeds: ");
    let delim: char = ' ';
    
    let expected: Vec<usize> = vec![79, 14, 55, 13];

    let res: Vec<usize> = parse_numbers_after_substring(input_string, substring, delim);
    assert_eq!(expected, res);
  }
}
