// concatenate a vector of usize into a single usize number
pub fn concat(vec: &[usize]) -> usize {
  let mut built_str: String = String::new();

  for num in vec {
    let num_str: String = num.to_string();
    built_str += &num_str;
  }

  return built_str.parse().unwrap();
}
