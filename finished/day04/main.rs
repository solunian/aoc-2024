use std::fs;

// coord to index
fn ctoi(y: usize, x: usize, ll: usize) -> usize {
  return y * ll + x;
}

fn main() {
  let input = fs::read_to_string("src/input.txt").unwrap();
  let c: Vec<char> = input.chars().filter(|x| *x != '\n').collect();
  let ll: usize = input.chars().position(|x| x == '\n').unwrap(); // must have a newline, line length

  let mut count = 0; // part 1
  let mut x_mas_count = 0; // part 2

  for i in 0..c.len() {
    let (y, x) = (i / ll, i % ll);

    // part 1
    if c[i] == 'X' {
      // +-horizontal, +-vertical, br tl, bl tr
      if x + 3 < ll && c[i + 1] == 'M' && c[i + 2] == 'A' && c[i + 3] == 'S' {
        count += 1;
      }
      if x >= 3 && c[i - 1] == 'M' && c[i - 2] == 'A' && c[i - 3] == 'S' {
        count += 1;
      }
      if y < ll - 3 && c[ctoi(y + 1, x, ll)] == 'M' && c[ctoi(y + 2, x, ll)] == 'A' && c[ctoi(y + 3, x, ll)] == 'S' {
        count += 1;
      }
      if y >= 3 && c[ctoi(y - 1, x, ll)] == 'M' && c[ctoi(y - 2, x, ll)] == 'A' && c[ctoi(y - 3, x, ll)] == 'S' {
        count += 1;
      }
      if y < ll - 3 && x + 3 < ll && c[ctoi(y + 1, x + 1, ll)] == 'M' && c[ctoi(y + 2, x + 2, ll)] == 'A' && c[ctoi(y + 3, x + 3, ll)] == 'S' {
        count += 1;
      }
      if y >= 3 && x >= 3 && c[ctoi(y - 1, x - 1, ll)] == 'M' && c[ctoi(y - 2, x - 2, ll)] == 'A' && c[ctoi(y - 3, x - 3, ll)] == 'S' {
        count += 1;
      }
      if y < ll - 3 && x >= 3 && c[ctoi(y + 1, x - 1, ll)] == 'M' && c[ctoi(y + 2, x - 2, ll)] == 'A' && c[ctoi(y + 3, x - 3, ll)] == 'S' {
        count += 1;
      }
      if y >= 3 && x + 3 < ll && c[ctoi(y - 1, x + 1, ll)] == 'M' && c[ctoi(y - 2, x + 2, ll)] == 'A' && c[ctoi(y - 3, x + 3, ll)] == 'S' {
        count += 1;
      }
    }

    // part 2
    if c[i] == 'A' && 0 < y && y < ll - 1 && 0 < x && x < ll - 1 {
      x_mas_count += match (c[ctoi(y - 1, x - 1, ll)], c[ctoi(y - 1, x + 1, ll)], c[ctoi(y + 1, x - 1, ll)], c[ctoi(y + 1, x + 1, ll)]) {
        ('M', 'M', 'S', 'S') | ('M', 'S', 'M', 'S') | ('S', 'S', 'M', 'M') | ('S', 'M', 'S', 'M') => 1,
        _ => 0
      };
    }
  }

  println!("part 1: {}", count);
  println!("part 2: {}", x_mas_count);
}