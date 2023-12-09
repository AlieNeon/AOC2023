use std::fs;

fn main() {
  const FILE_PATH: &str = "input.txt";
  println!(
    "Welcome to AOC2023 day NUMBER, first we will read the file {}",
    FILE_PATH
  );
  let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

  println!("And we parse the file!");
  let data = parse(&contents);
  println!("First thing: {}", calculate_1(&data));
  println!("Second thing: {}", calculate_2(&data));
}

fn parse(input: &str) -> Vec<Vec<i32>> {
  input
    .lines()
    .map(|line| {
      line
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
    })
    .collect()
}

fn calculate_next(secuence: &Vec<i32>) -> i32 {
  if secuence.iter().all(|n| n == &0) {
    return 0;
  }
  secuence.last().unwrap()
    + calculate_next(
      &secuence
        .iter()
        .zip(secuence.iter().skip(1))
        .map(|(p, n)| n - p)
        .collect(),
    )
}

fn calculate_1(secuences: &Vec<Vec<i32>>) -> i32 {
  secuences.iter().map(calculate_next).sum()
}

fn calculate_2(secuences: &Vec<Vec<i32>>) -> i32 {
  secuences.iter().map(|sec| calculate_next(&sec.iter().rev().map(|n| *n).collect())).sum()
}

#[cfg(test)]
mod test {
  const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
  use super::*;
  #[test]
  fn test1() {
    let tst = parse(INPUT);
    assert_eq!(calculate_1(&tst), 114);
  }
  #[test]
  fn test2() {
    let tst = parse("10  13  16  21  30  45");
    assert_eq!(calculate_2(&tst), 5);
  }
}
