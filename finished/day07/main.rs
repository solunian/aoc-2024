use std::fs;
use itertools::Itertools;

fn test_combinations_pt1(eq: &Vec<i64>) -> bool {
  let comb_limit: i64 = 1 << (eq.len() - 2); // n - 1 of values because in between as operators
  // 0 is +, 1 is *
  for comb_c in 0..comb_limit {
    let mut sum = eq[1];
    for i in 2..eq.len() {
      sum = match (comb_c >> (i - 2)) & 1 {
        0 => sum + eq[i],
        1 => sum * eq[i],
        _ => sum
      };
    }

    if sum == eq[0] {
      return true;
    }
  }

  return false;  
}

fn concat(a: i64, b: i64) -> i64 {
  a * 10_i64.pow((b as f64).log10().floor() as u32 + 1) + b
}

// fn permutations_with_replacement(range, k) -> Vec<Vec<i64>> {}

fn test_combinations_pt2(eq: &Vec<i64>) -> bool {
  let permutations = itertools::repeat_n(0..3, eq.len() - 2).multi_cartesian_product(); // essential permutations with replacement
  // 0 is +, 1 is *, 2 is ||
  for p in permutations {
    let mut sum = eq[1];
    for i in 2..eq.len() {
      match p[i - 2] {
        0 => sum += eq[i],
        1 => sum *= eq[i],
        2 => sum = concat(sum, eq[i]),
        _ => ()
      };
    }

    if sum == eq[0] {
      return true;
    }
  }

  return false;  
}

fn main() {
  let input = fs::read_to_string("src/input.txt").unwrap();
  let mut eqs: Vec<Vec<i64>> = Vec::new();
  for line in input.lines() {
    if line == "" { continue; }
    let mut eq: Vec<i64> = Vec::new();
    let spl: Vec<&str> = line.split(": ").collect();
    eq.push(spl[0].parse().unwrap());

    eq.extend(spl[1].split(" ").map(|x| x.parse::<i64>().unwrap()));
    eqs.push(eq);
  }

  let mut sum: i64 = 0;
  let mut sum_pt2: i64 = 0;
  for eq in eqs {
    if test_combinations_pt1(&eq) {
      sum += eq[0];
    }
    if test_combinations_pt2(&eq) {
      sum_pt2 += eq[0];
    }
  }

  println!("part 1: calibration result -> {}", sum);
  println!("part 2: total calibration result -> {}", sum_pt2);
}