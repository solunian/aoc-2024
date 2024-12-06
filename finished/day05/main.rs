use std::{cmp, collections::{HashMap, HashSet}, fs};

fn does_include(slice: &[i32], rule: &HashSet<i32>) -> bool {
  for i in slice.iter() {
    if rule.contains(i) {
      return true;
    }
  }
  return false;
}

fn fix_update(update: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
  let mut fixed = update.clone();

  fixed.sort_by(|a, b| {
    if rules.contains_key(a) && rules.get(a).unwrap().contains(b) {
      return cmp::Ordering::Less;
    } else if rules.contains_key(b) && rules.get(b).unwrap().contains(a) {
      return cmp::Ordering::Greater;
    } else {
      return cmp::Ordering::Equal;
    }
  });
  
  return fixed;
}

fn main() {
  let input = fs::read_to_string("src/input.txt").unwrap();
  let (rules, updates) = {
    let spl: Vec<&str> = input.split("\n\n").collect();

    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for rule in spl[0].lines() {
      let rule_spl: Vec<i32> = rule.split("|").map(|x| x.parse().unwrap()).collect();
      if rules.contains_key(&rule_spl[0]) {
        rules.get_mut(&rule_spl[0]).unwrap().insert(rule_spl[1]);
      } else {
        let mut new_set = HashSet::new();
        new_set.insert(rule_spl[1]);
        rules.insert(rule_spl[0], new_set);
      }
    }

    let mut updates: Vec<Vec<i32>> = Vec::new();
    for update in spl[1].lines() {
      updates.push(update.split(",").map(|x| x.parse().unwrap()).collect());
    }

    (rules, updates)
  };

  let mut sum = 0;
  let mut fix_sum = 0;

  for update in &updates {
    let mut rev: Vec<i32> = update.clone();
    rev.reverse();
    let mut is_valid = true;
    for (i, val) in rev.iter().enumerate() {
      if rules.contains_key(val) && does_include(&rev[i + 1..], &rules[val]) {
        is_valid = false;
        break;
      }
    }
    if is_valid {
      sum += update[update.len() / 2];
    } else {
      let fixed = fix_update(&update, &rules);
      fix_sum += fixed[fixed.len() / 2];
    }
  }

  println!("part 1: sum middle -> {}", sum);
  println!("part 2: fixed sum middle -> {}", fix_sum);

}