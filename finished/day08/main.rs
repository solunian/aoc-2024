use std::collections::HashMap;
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

  // fn val_in(self, map: &Vec<Vec<bool>>) -> bool {
  //   map[self.1 as usize][self.0 as usize]
  // }

  // fn mut_val_in(self, map: &mut Vec<Vec<bool>>) -> &mut bool {
  //   &mut map[self.1 as usize][self.0 as usize]
  // }

  fn find_antinodes(self, other: Self) -> Vec<P> {
    let dx = self.0 - other.0;
    let dy = self.1 - other.1;

    vec![P(self.0 + dx, self.1 + dy), P(other.0 - dx, other.1 - dy)]
  }

  fn find_antinodes_pt2(self, other: Self, w: i32, h: i32) -> Vec<P> {
    let dx = self.0 - other.0;
    let dy = self.1 - other.1;

    let mut antis = Vec::new();
    antis.push(self);
    antis.push(other);
    let mut self_antis = P(self.0 + dx, self.1 + dy);
    let mut other_antis = P(other.0 - dx, other.1 - dy);
    while self_antis.within(w, h) {
      antis.push(self_antis);
      self_antis.0 += dx;
      self_antis.1 += dy;
    }
    while other_antis.within(w, h) {
      antis.push(other_antis);
      other_antis.0 -= dx;
      other_antis.1 -= dy;
    }

    antis
  }
}

fn main() {
  let input = fs::read_to_string("src/input.txt").unwrap();
  let h: i32 = input.lines().collect::<Vec<_>>().len() as i32;
  let w: i32 = (input.chars().collect::<Vec<_>>().len() as i32 / h) - 1; // -1 for \n char

  let mut map: HashMap<char, Vec<P>> = HashMap::new();

  for (y, line) in input.lines().enumerate() {
    for (x, c) in line.chars().enumerate() {
      if c != '.' {
        let coord = P(x as i32, y as i32);
        if !map.contains_key(&c) {
          let coords = vec![coord];
          map.insert(c, coords);
        } else {
          map.get_mut(&c).unwrap().push(coord);
        }
      }
    }
  }

  let mut antinodes: HashSet<P> = HashSet::new();
  let mut antinodes_pt2: HashSet<P> = HashSet::new();
  for (_, coords) in map {
    for i in 0..coords.len() {
      for j in i+1..coords.len() {
        let antis = coords[i].find_antinodes(coords[j]);
        let antis_pt2 = coords[i].find_antinodes_pt2(coords[j], w, h);
        for i in antis {
          if i.within(w, h) {
            antinodes.insert(i);
          }
        }
        for i in antis_pt2 {
          antinodes_pt2.insert(i);
        }
      }
    }
  }

  println!("part 1: antinodes count -> {}", antinodes.len());
  println!("part 2: extra strength antinodes count -> {}", antinodes_pt2.len());

}