use std::fs;
use std::collections::HashMap;

fn main() {
  let mut v1: Vec<i32> = Vec::new();
  let mut v2: Vec<i32> = Vec::new();

  let mut freq = HashMap::new();
  
  let contents = fs::read_to_string("src/input.txt").unwrap();
  for line in contents.lines() {
    let spl: Vec<i32> = line.trim().split("   ").map(|x| x.parse::<i32>().unwrap()).collect();
    v1.push(spl[0]);
    v2.push(spl[1]);

    freq.entry(spl[1]).or_insert(0);
    freq.insert(spl[1], freq[&spl[1]] + 1);
  }

  v1.sort();
  v2.sort();

  let mut sum_distances = 0;
  let mut similarity_score = 0;
  
  for i in 0..v1.len() {
    sum_distances += (v2[i] - v1[i]).abs();
    similarity_score += v1[i] * freq.get(&v1[i]).unwrap_or(&0);
  }

  println!("part 1 -> sum_distances: {}", sum_distances);
  println!("part 2 -> similarity_score: {}", similarity_score);
}