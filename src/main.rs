use std::fs;

fn main() {
  let input: Vec<char> = fs::read_to_string("src/input.txt").unwrap().chars().collect();
  let mut is_file = true;
  let mut filesystem: Vec<i32> = vec![];
  let mut used_space: usize = 0;

  for i in 0..input.len() {
    let rep = input[i].to_digit(10).unwrap();
    if is_file {
      for _ in 0..rep {
        filesystem.push((i as i32) / 2); // id is half the index because it swaps back and forth
      }
      used_space += rep as usize;
    } else {
      for _ in 0..rep {
        filesystem.push(-1);
      }
    }
    is_file = !is_file;
  }

  let mut checksum: i64 = 0;
  for i in 0..used_space {
    if filesystem[i] == -1 {
      for j in (0..filesystem.len()).rev() {
        if filesystem[j] != -1 {
          (filesystem[i], filesystem[j]) = (filesystem[j], filesystem[i]);
          break;
        }
      }
    }
    
    checksum += i as i64 * filesystem[i] as i64;
  }

  println!("part 1: filesystem checksum -> {}", checksum);
}