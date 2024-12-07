use std::{collections::HashSet, fs};
use std::ops::Add;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct P(i32, i32);

impl Add for P {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self (self.0 + other.0, self.1 + other.1)
  }
}

impl P {
  fn within(self, w: i32, h: i32) -> bool {
    0 <= self.0 && self.0 < w && 0 <= self.1 && self.1 < h
  }

  fn val_in(self, map: &Vec<Vec<bool>>) -> bool {
    map[self.1 as usize][self.0 as usize]
  }

  fn mut_val_in(self, map: &mut Vec<Vec<bool>>) -> &mut bool {
    &mut map[self.1 as usize][self.0 as usize]
  }
}

fn main() {
  let input = fs::read_to_string("src/input.txt").unwrap();
  let mut map: Vec<Vec<bool>> = Vec::new();
  let mut guard = P(0, 0);

  let dirs = vec![P(0, -1), P(1, 0), P(0, 1), P(-1, 0)];
  let mut dir = 0;

  // process input
  for (y, line) in input.lines().enumerate() {
    let mut row: Vec<bool> = Vec::new();
    for (x, c) in line.chars().enumerate() {
      match c {
        '#' => row.push(true),
        '^' => {
          guard = P(x as i32, y as i32);
          row.push(false);          
        }
        _ => row.push(false)
      }
    }
    map.push(row);
  }
  
  let h: i32 = map.len() as i32;
  let w = map[0].len() as i32;

  let mut spots_covered: HashSet<P> = HashSet::new();

  let guard_clone = guard.clone(); // for part 2

  while guard.within(w, h) {
    spots_covered.insert(guard);
    let new_guard = guard + dirs[dir];
    if new_guard.within(w, h) && new_guard.val_in(&map) {
      dir = (dir + 1) % 4;
    } else {
      guard = new_guard;
    }
  }

  println!("part 1: distinct positions -> {}", spots_covered.len());

  // part 2
  let mut looped_count = 0; // count for whenever theres a cycle
  let mut path: HashSet<(P, usize)> = HashSet::new(); // all traveled points/directions

  for spot in spots_covered { // shorten search to only spots that the guard will travel to, taken from part 1
    // reset vals for guard
    guard = guard_clone;
    dir = 0;

    if guard == spot || spot.val_in(&map) { // can't be where guard first starts, or where obstacles currently are
      continue;
    }

    *spot.mut_val_in(&mut map) = true;

    // run simulation
    while guard.within(w, h) {
      if path.contains(&(guard, dir)) { // hits same path means path loop
        looped_count += 1;
        break;
      }

      // add spot/dir instance to set
      path.insert((guard, dir));

      // move and dir change
      let new_guard = guard + dirs[dir];
      if new_guard.within(w, h) && new_guard.val_in(&map) {
        dir = (dir + 1) % 4;
      } else {
        guard = new_guard;
      }
    }

    // unset the map, clear traveled path
    *spot.mut_val_in(&mut map) = false;
    path.clear();
  }

  println!("part 2: loop stuff -> {}", looped_count);
}