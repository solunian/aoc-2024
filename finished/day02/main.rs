use std::fs;

fn is_safe(r: &Vec<i32>) -> bool {
  if r.len() <= 1 {
    return true;
  }

  let is_increasing = r[1] > r[0];

  for i in 0..r.len() - 1 {
    if is_increasing != (r[i + 1] > r[i]) || (r[i + 1] - r[i]).abs() > 3 || (r[i + 1] - r[i]).abs() < 1 {
      return false;
    }
  }

  return true;
}

fn is_dampner_safe(r: &Vec<i32>) -> bool {
  if r.len() <= 1 {
    return true;
  }

  for i in 0..r.len() {
    let mut rclone = r.clone();
    rclone.remove(i);

    if is_safe(&rclone) {
      return true;
    }
  }

  return false;
}

fn main() {
  let content = fs::read_to_string("src/input.txt").unwrap();
  let mut safe_report_count = 0;
  let mut dampner_safe_count = 0;

  for line in content.lines() {
    let report: Vec<i32> = line.trim().split(" ").map(|x| x.parse().unwrap()).collect();

    if is_safe(&report) {
      safe_report_count += 1;
    }
    if is_dampner_safe(&report) {
      dampner_safe_count += 1;
    }
  }

  println!("part 1 -> safe_report_count: {}", safe_report_count);
  println!("part 2 -> safe_dampner_count: {}", dampner_safe_count);
}