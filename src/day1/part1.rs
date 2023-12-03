

pub fn partone() {
  let data: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
  let mut sum: i32 = 0;


  let mut temp_str: String = "".to_owned();
  let mut is_new_line: bool = false;

  for c in data.chars() {
    println!("{}", c);
    if c.is_numeric() && is_new_line == false {
      temp_str.push(c);
      println!("{}", c);
      // do something with `c`
    } else if c == '\n' {
      let temp_int: i32 = temp_str.parse().unwrap();
      sum += temp_int;
      temp_str = "".to_owned();
    }
    println!("sum: {}, temp_str: {}", sum, temp_str);

      // do something with `c`
  }
}
