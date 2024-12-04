use std::fs;

fn mul_from_str(s: &str) -> Option<i32> {
  if s.len() < 8 || s.len() > 12 {
    return None;
  }

  if s.starts_with("mul(") && s.ends_with(")") {
    // splits part between (...) by a comma, then parses into collection of i32
    let spl: Vec<Result<i32, _>> = s[4..s.len() - 1].split(",").map(|val| val.parse()).collect();
    if spl.len() != 2 {
      return None;
    }

    return match (&spl[0], &spl[1]) {
      (Ok(a), Ok(b)) => Some(a * b),
      _ => None
    };
  }

  return None;
}

fn main() {
  let input = fs::read_to_string("src/input.txt").unwrap(); // guarantee no error

  // for part 1
  let mut sum = 0;

  // for part 2
  let mut enabled_sum = 0;
  let mut enabled = true;

  for i in 0..input.len() { // len can be used because its marked by bytes, and the 'm' and ')' chars are one byte in unicode
    if input[i..].starts_with("do()") {
      enabled = true;
    }
    if input[i..].starts_with("don't()") {
      enabled = false;
    }

    if &input[i..i + 1] == "m" { // find m to start
      for j in i + 1..input.len() {
        if &input[j..j + 1] == ")" { // find ) to end
          let val = mul_from_str(&input[i..=j]).unwrap_or(0);
          sum += val; // part 1
          if enabled {
            enabled_sum += val; // part 2
          }
          break;
        }
      }
    }
  }

  println!("part 1: mul -> {}", sum);
  println!("part 2: enabled -> {}", enabled_sum);
}