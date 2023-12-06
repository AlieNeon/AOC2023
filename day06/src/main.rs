use std::fs;

fn main() {
  const FILE_PATH: &str = "input.txt";
  println!(
    "Welcome to AOC2023 day 6, first we will read the file {}",
    FILE_PATH
  );
  let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

  println!("And we parse the file!");
  println!("First thing: {}", calculate_1(&parse1(&contents)));
  println!("Second thing: {}", calculate(&parse2(&contents)));
}

fn parse1(input: &str) -> Vec<(u64, u64)> {
  let mut lines = input.lines().map(|line| {
    line
      .split(": ")
      .nth(1)
      .unwrap()
      .trim()
      .split_ascii_whitespace()
      .map(|n| n.parse().expect("NAN"))
  });
  let time = lines.next().unwrap();
  let mut distance = lines.next().unwrap();
  let mut output = Vec::new();
  time.for_each(|n| output.push((n, distance.next().unwrap())));
  output
}

fn parse2(input: &str) -> (u64, u64) {
  let mut lines = input.lines().map(|line| {
    line
      .split(": ")
      .nth(1)
      .unwrap()
      .chars()
      .filter(|c| !c.is_ascii_whitespace())
      .collect::<String>()
  });
  (
    lines.next().unwrap().parse().expect("NAN"),
    lines.next().unwrap().parse().expect("NAN"),
  )
}

fn calculate_1(races: &Vec<(u64, u64)>) -> u64 {
  races
    .iter()
    .map(|race| calculate(race))
    .fold(1, |a, n| a * n)
}

fn calculate((time, distance): &(u64, u64)) -> u64 {
  (1..*time)
    .map(|p| p * (time - p))
    .filter(|n| n > &distance)
    .count()
    .try_into()
    .unwrap()
}

#[cfg(test)]
mod test {
  const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;
  use super::*;
  #[test]
  fn test1() {
    let tst = parse1(INPUT);
    assert_eq!(calculate_1(&tst), 288);
  }
  #[test]
  fn test2() {
    let tst = parse2(INPUT);
    assert_eq!(calculate(&tst), 71503);
  }
}
