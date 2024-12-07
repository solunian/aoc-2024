// i was actually being so stupid when writing this.
// i mean it works? but it took several minutes on the input.txt
// why did i use a vec instead of a set

use std::{collections::HashSet, fs};
use std::ops::Add;


#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct P(i32, i32);

impl Add for P {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self (
      self.0 + other.0,
      self.1 + other.1,
    )
  }
}

fn main() {
  let input = fs::read_to_string("src/input.txt").unwrap();
  let mut map: HashSet<P> = HashSet::new();
  let (w, h) = (input.lines().nth(0).unwrap().len() as i32, input.lines().collect::<Vec<_>>().len() as i32);
  let mut guard = P(0, 0);

  let dirs = vec![P(0, -1), P(1, 0), P(0, 1), P(-1, 0)];
  let mut dir = 0;

  for (row, line) in input.lines().enumerate() {
    for (col, c) in line.chars().enumerate() {
      match c {
        '#' => {
          map.insert(P(col as i32, row as i32));
        }
        '^' => {
          guard = P(col as i32, row as i32);
        }
        _ => ()
      }
    }
  }

  let guard_clone = guard.clone(); // for part 2
  let mut spots_covered: HashSet<P> = HashSet::new();

  while 0 <= guard.0 && guard.0 < w && 0 <= guard.1 && guard.1 < h {
    if map.contains(&(guard + dirs[dir])) {
      dir = (dir + 1) % 4;
    } else {
      spots_covered.insert(guard);
      guard = guard + dirs[dir];
    }
  }

  println!("part 1: distinct positions -> {}", spots_covered.len());

  // part 2
  let mut looped_count = 0;
  let mut path: Vec<P> = Vec::new();
  let mut directions: Vec<usize> = Vec::new();

  for row in 0..h {
    for col in 0..w {

      guard = guard_clone;
      dir = 0;
      let mut looped = false;

      if !spots_covered.contains(&P(col, row)) || map.contains(&P(col, row)) || guard == P(col, row) {
        continue;
      }

      map.insert(P(col, row));

      while 0 <= guard.0 && guard.0 < w && 0 <= guard.1 && guard.1 < h {
        for i in 1..path.len() {
          if path[i] == guard && directions[i] == dir {
            looped = true;
            break;
          }
        }

        // if path.len() > 1 {
        //   if path[path.len() - 1] == guard && directions[directions.len() - 1] == dir {
        //     looped = true;
        //   }
        // }

        if looped {
          println!("looped");
          looped_count += 1;
          break;
        }

        path.push(guard);
        directions.push(dir);

        if map.contains(&(guard + dirs[dir])) {
          dir = (dir + 1) % 4;
        } else {
          guard = guard + dirs[dir];
        }
      }

      map.remove(&P(col, row));
      path.clear();
      directions.clear();

      println!("finish coord {} {}", col, row);
    }
  }

  println!("part 2: loop stuff -> {}", looped_count);
  
}