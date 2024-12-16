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


  let mut files: Vec<i32> = Vec::new(); // the index represents their id, sizes
  let mut files_idx: Vec<usize> = Vec::new();
  let mut spaces: Vec<i32> = Vec::new(); // size of each
  let mut spaces_idx: Vec<usize> = Vec::new();

  let mut is_file = true;
  let mut current_spot = 0;

  for i in 0..input.len() {
    let rep = input[i].to_digit(10).unwrap() as i32;
    if is_file {
      files.push(rep);
      files_idx.push(current_spot);
    } else {
      spaces.push(rep);
      spaces_idx.push(current_spot);
    }
    current_spot += rep as usize;
    is_file = !is_file;
  }

  let mut i = 0;
  while i < spaces.len() {
    let mut ext_id = files.len() as i32 - 1;

    while ext_id >= 0 {
      let id: usize = ext_id as usize;

      let fsize = files[id];
      let ssize = spaces[i];

      if files_idx[id] > spaces_idx[i] {
        if fsize == ssize {
          files_idx[id] = spaces_idx[i];
          break;
        } else if fsize < ssize {
          files_idx[id] = spaces_idx[i];
          spaces.insert(i + 1, ssize - fsize);
          spaces_idx.insert(i + 1, spaces_idx[i] + fsize as usize);
          break;
        }
      }

      ext_id -= 1; 
    }

    i += 1;
  }

  let mut checksum_p2: i64 = 0;
  for (id, file) in files.iter().enumerate() {
    for i in 0..*file as i64 {
      checksum_p2 += id as i64 * (files_idx[id] as i64 + i);
    }
  }

  println!("part 2: lol what contiguous files -> {}", checksum_p2);

}