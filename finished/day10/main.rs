use std::collections::HashSet;
use std::fs;
use std::ops::Add;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug, Default)]
struct P {
  x: i32,
  y: i32,
}

impl Add for P {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    P::new(self.x + other.x, self.y + other.y)
  }
}

impl P {
  fn new(x: i32, y: i32) -> Self {
    Self{x, y}
  }

  fn check(&self, elevation: i32, map: &Vec<Vec<i32>>) -> Vec<P> {
    let mut moves: Vec<P> = Vec::new();

    let x = self.x as usize;
    let y = self.y as usize;

    if y > 0 && elevation + 1 == map[y - 1][x] {
      moves.push(P::new(0, -1));
    }
    if y < map.len() - 1 && elevation + 1 == map[y + 1][x] {
      moves.push(P::new(0, 1));
    }
    if x > 0 && elevation + 1 == map[y][x - 1] {
      moves.push(P::new(-1, 0));
    }
    if x < map[0].len() - 1 && elevation + 1 == map[y][x + 1] {
      moves.push(P::new(1, 0));
    }

    moves
  }
}

fn main() {
  let input = fs::read_to_string("src/input.txt").unwrap();

  let mut map: Vec<Vec<i32>> = Vec::new();
  let mut trailheads: Vec<P> = Vec::new();

  for (y, line) in input.lines().enumerate() {
    let spl: Vec<i32> = line.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();

    for (x,&val) in spl.iter().enumerate() {
      if val == 0 {
        trailheads.push(P::new(x as i32, y as i32));
      }
    }
    map.push(spl);
  }

  let mut reached_nines_count = 0;
  let mut trailscount = 0; // part 2
  for head in trailheads {
    let mut stack: Vec<(i32, P)> = vec![(0, head)];
    let mut reached_nines: HashSet<P> = HashSet::new();
    while !stack.is_empty() {
      let (elevation, point) = stack.pop().unwrap();
      if elevation == 9 {
        reached_nines.insert(point);
        trailscount += 1;
      } else {
        let moves = point.check(elevation, &map);
        for step in moves {
          stack.push((elevation + 1, point + step));
        }
      }
    }
    reached_nines_count += reached_nines.len() as i32;
  }

  println!("part 1: 9s reached -> {reached_nines_count}");
  println!("part 2: total trails -> {trailscount}");
}